use std::{
    convert::TryFrom,
    error,
    error::Error,
    time::{Duration, SystemTime},
};

use git2::{Config, ConfigEntries};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Eq, PartialEq)]
pub struct Author {
    name: String,
    email: String,
}

impl Author {
    #[must_use]
    pub fn new(name: &str, email: &str) -> Author {
        Author {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub fn email(&self) -> String {
        self.email.clone()
    }
}

#[must_use]
pub fn get_author_configuration(config: &Config) -> std::option::Option<Vec<Author>> {
    let right_less_than_left = |pair: (Duration, Duration)| -> bool { pair.0.lt(&pair.1) };
    let i64_into_u64 = |x| u64::try_from(x).map_err(Box::<dyn Error>::from);
    let left_time_right_duration = |expires_after_time| {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(Box::from)
            .map(|time_since_epoch| (time_since_epoch, expires_after_time))
    };
    let is_true = |x: &bool| true.eq(&x);
    let replace_with_coauthors = |_| defined_coauthors(config);

    config
        .get_i64("pb.author.expires")
        .map_err(Box::from)
        .and_then(i64_into_u64)
        .map(Duration::from_secs)
        .and_then(left_time_right_duration)
        .map(right_less_than_left)
        .ok()
        .filter(is_true)
        .map(replace_with_coauthors)
}

fn defined_coauthors(config: &Config) -> Vec<Author> {
    let mut authors: Vec<Author> = vec![];

    while let Ok(true) = config_defined(config, &format!("pb.author.coauthors.{}.*", authors.len()))
    {
        let email = if let Ok(email) =
            config.get_str(&format!("pb.author.coauthors.{}.email", authors.len()))
        {
            email
        } else {
            return authors;
        };

        let name = match config.get_str(&format!("pb.author.coauthors.{}.name", authors.len())) {
            Ok(name) => name,
            _ => return authors,
        };

        authors.push(Author::new(name, email))
    }

    authors
}

fn config_defined(config: &Config, config_key: &str) -> Result<bool> {
    let more_than_1_config_variable = |x: ConfigEntries| x.count() > 1;
    config
        .entries(Some(config_key))
        .map(more_than_1_config_variable)
        .map_err(Box::from)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn has_an_author() {
        let author = Author::new("The Name", "email@example.com");

        assert_eq!(author.name(), "The Name");
        assert_eq!(author.email(), "email@example.com");
    }
}

#[cfg(test)]
mod tests_able_to_load_config_from_git {
    use std::{
        convert::TryFrom,
        ops::{Add, Sub},
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use pretty_assertions::assert_eq;

    use crate::author::{get_author_configuration, Author};
    use pb_hook_test_helper::make_config;

    #[test]
    fn there_is_no_author_config_if_it_has_expired() {
        let mut config = make_config();
        let now_minus_10 = epoch_with_offset(subtract_10_seconds);

        config
            .set_i64("pb.author.expires", now_minus_10)
            .expect("Failed to set config");

        let snapshot = config.snapshot().expect("Failed to snapshot config");

        let actual = get_author_configuration(&snapshot);
        let expected = None;
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn there_is_a_config_if_the_config_has_not_expired() {
        let mut config = make_config();
        let now_plus_10_seconds = epoch_with_offset(add_10_seconds);

        config
            .set_i64("pb.author.expires", now_plus_10_seconds)
            .expect("Failed to set config");

        let snapshot = config.snapshot().expect("Failed to snapshot config");

        let actual = get_author_configuration(&snapshot);
        let expected: Option<Vec<Author>> = Some(vec![]);
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn we_get_author_config_back_if_there_is_any() {
        let mut config = make_config();
        let now_plus_10_seconds = epoch_with_offset(add_10_seconds);
        config
            .set_i64("pb.author.expires", now_plus_10_seconds)
            .expect("Failed to set config");

        config
            .set_str("pb.author.coauthors.0.email", "annie@example.com")
            .expect("Failed to set config");

        config
            .set_str("pb.author.coauthors.0.name", "Annie Example")
            .expect("Failed to set config");

        let snapshot = config.snapshot().expect("Failed to snapshot config");

        let actual = get_author_configuration(&snapshot);
        let expected = Some(vec![Author::new("Annie Example", "annie@example.com")]);
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    fn add_10_seconds(x: Duration) -> Duration {
        x.add(Duration::from_secs(10))
    }

    fn subtract_10_seconds(x: Duration) -> Duration {
        x.sub(Duration::from_secs(10))
    }

    fn into_seconds(x: Duration) -> u64 {
        x.as_secs()
    }

    #[test]
    fn we_get_multiple_authors_back_if_there_are_multiple() {
        let mut config = make_config();
        let now_plus_10_seconds = epoch_with_offset(add_10_seconds);
        config
            .set_i64("pb.author.expires", now_plus_10_seconds)
            .expect("Failed to set config");

        config
            .set_str("pb.author.coauthors.0.email", "annie@example.com")
            .expect("Failed to set config");

        config
            .set_str("pb.author.coauthors.0.name", "Annie Example")
            .expect("Failed to set config");

        config
            .set_str("pb.author.coauthors.1.email", "joe@example.com")
            .expect("Failed to set config");

        config
            .set_str("pb.author.coauthors.1.name", "Joe Bloggs")
            .expect("Failed to set config");

        let snapshot = config.snapshot().expect("Failed to snapshot config");

        let actual = get_author_configuration(&snapshot);
        let expected = Some(vec![
            Author::new("Annie Example", "annie@example.com"),
            Author::new("Joe Bloggs", "joe@example.com"),
        ]);
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    fn epoch_with_offset(x: fn(Duration) -> Duration) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(x)
            .map(into_seconds)
            .map(i64::try_from)
            .expect("Failed to get Unix Epoch")
            .expect("Convert epoch to int")
    }
}
