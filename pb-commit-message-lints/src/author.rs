use std::{
    convert::TryFrom,
    error::Error,
    time::{Duration, SystemTime},
};

use crate::VcsConfig;

#[derive(Debug, Eq, PartialEq)]
pub struct Author {
    name: String,
    email: String,
}

impl Author {
    #[must_use]
    pub fn new(name: &str, email: &str) -> Author {
        Author {
            name: name.into(),
            email: email.into(),
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
pub fn get_author_configuration(config: &dyn VcsConfig) -> std::option::Option<Vec<Author>> {
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
        .ok_or_else(|| "No author expiry date".into())
        .and_then(i64_into_u64)
        .map(Duration::from_secs)
        .and_then(left_time_right_duration)
        .map(right_less_than_left)
        .ok()
        .filter(is_true)
        .map(replace_with_coauthors)
}

fn defined_coauthors(config: &dyn VcsConfig) -> Vec<Author> {
    let mut authors: Vec<Author> = vec![];

    while config
        .get_str(&format!("pb.author.coauthors.{}.email", authors.len()))
        .is_some()
    {
        let email = if let Some(email) =
            config.get_str(&format!("pb.author.coauthors.{}.email", authors.len()))
        {
            email
        } else {
            return authors;
        };

        let name = match config.get_str(&format!("pb.author.coauthors.{}.name", authors.len())) {
            Some(name) => name,
            _ => return authors,
        };

        authors.push(Author::new(name, email))
    }

    authors
}

#[cfg(test)]
mod tests_author {
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

    use crate::{
        author::{get_author_configuration, Author},
        config::InMemoryVcs,
    };
    use std::collections::HashMap;

    #[test]
    fn there_is_no_author_config_if_it_has_expired() {
        let now_minus_10 = epoch_with_offset(subtract_10_seconds);

        let mut i64_configs = HashMap::new();
        i64_configs.insert("pb.author.expires".into(), now_minus_10);
        let git2_config = InMemoryVcs::new(HashMap::new(), HashMap::new(), i64_configs);

        let actual = get_author_configuration(&git2_config);
        let expected = None;
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn there_is_a_config_if_the_config_has_not_expired() {
        let now_plus_10_seconds = epoch_with_offset(add_10_seconds);

        let mut i64_configs = HashMap::new();
        i64_configs.insert("pb.author.expires".into(), now_plus_10_seconds);
        let git2_config = InMemoryVcs::new(HashMap::new(), HashMap::new(), i64_configs);

        let actual = get_author_configuration(&git2_config);

        let expected: Option<Vec<Author>> = Some(vec![]);
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn we_get_author_config_back_if_there_is_any() {
        let now_plus_10_seconds = epoch_with_offset(add_10_seconds);

        let mut i64_configs = HashMap::new();
        i64_configs.insert("pb.author.expires".into(), now_plus_10_seconds);
        let mut str_configs = HashMap::new();
        str_configs.insert(
            "pb.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        str_configs.insert("pb.author.coauthors.0.name".into(), "Annie Example".into());
        let git2_config = InMemoryVcs::new(HashMap::new(), str_configs, i64_configs);

        let actual = get_author_configuration(&git2_config);

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
        let now_plus_10_seconds = epoch_with_offset(add_10_seconds);
        let mut i64_configs = HashMap::new();
        i64_configs.insert("pb.author.expires".into(), now_plus_10_seconds);
        let mut str_configs = HashMap::new();
        str_configs.insert(
            "pb.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        str_configs.insert("pb.author.coauthors.0.name".into(), "Annie Example".into());
        str_configs.insert(
            "pb.author.coauthors.1.email".into(),
            "joe@example.com".into(),
        );
        str_configs.insert("pb.author.coauthors.1.name".into(), "Joe Bloggs".into());
        let git2_config = InMemoryVcs::new(HashMap::new(), str_configs, i64_configs);

        let actual = get_author_configuration(&git2_config);
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
