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
pub fn get_coauthor_configuration(config: &dyn VcsConfig) -> std::option::Option<Vec<Author>> {
    config
        .get_i64("pb.author.expires")
        .ok_or_else(|| "No author expiry date".into())
        .and_then(i64_into_u64)
        .map(Duration::from_secs)
        .and_then(time_and_now)
        .map(is_after)
        .ok()
        .filter(bool::clone)
        .map(partial!(replace_with_coauthors => _, config))
}

fn time_and_now(expires_after_time: Duration) -> Result<(Duration, Duration), Box<dyn Error>> {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(Box::from)
        .map(partial!(duration_tuple => expires_after_time, _))
}

fn duration_tuple(
    expires_after_time: Duration,
    time_since_epoch: Duration,
) -> (Duration, Duration) {
    (time_since_epoch, expires_after_time)
}

fn replace_with_coauthors(_: bool, config: &dyn VcsConfig) -> Vec<Author> {
    defined_coauthors(config)
}

fn i64_into_u64(x: i64) -> Result<u64, Box<dyn Error>> {
    u64::try_from(x).map_err(Box::<dyn Error>::from)
}

fn is_after((point, comparison): (Duration, Duration)) -> bool {
    point.lt(&comparison)
}

fn defined_coauthors(config: &dyn VcsConfig) -> Vec<Author> {
    get_config_names(config)
        .iter()
        .zip(get_config_emails(config))
        .filter_map(tuple_to_author)
        .collect()
}

fn tuple_to_author(a: (&Option<&str>, Option<&str>)) -> Option<Author> {
    match a {
        (Some(name), Some(email)) => Some(Author::new(name, email)),
        _ => None,
    }
}

fn get_config_names(config: &dyn VcsConfig) -> Vec<Option<&str>> {
    get_config_values(config, "name")
}

fn get_config_emails(config: &dyn VcsConfig) -> Vec<Option<&str>> {
    get_config_values(config, "email")
}

#[allow(clippy::maybe_infinite_iter)]
fn get_config_values<'a>(config: &'a dyn VcsConfig, key: &str) -> Vec<Option<&'a str>> {
    (0..)
        .take_while(|x| config_id_exists(config, *x))
        .map(partial!(get_from_config => config, key, _))
        .collect()
}

fn get_from_config<'a>(config: &'a dyn VcsConfig, key: &str, x: i32) -> Option<&'a str> {
    config.get_str(&format!("pb.author.coauthors.{}.{}", x, key))
}

fn config_id_exists(config: &dyn VcsConfig, id: i32) -> bool {
    read_email_from_config(config, id).is_some()
}

fn read_email_from_config(config: &'_ dyn VcsConfig, id: i32) -> Option<&'_ str> {
    config.get_str(&format!("pb.author.coauthors.{}.email", id))
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
        collections::HashMap,
        convert::TryFrom,
        ops::{Add, Sub},
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use pretty_assertions::assert_eq;

    use crate::{
        author::{get_coauthor_configuration, Author},
        config::InMemoryVcs,
    };

    #[test]
    fn there_is_no_author_config_if_it_has_expired() {
        let now_minus_10 = epoch_with_offset(subtract_10_seconds);

        let mut i64_configs = HashMap::new();
        i64_configs.insert("pb.author.expires".into(), now_minus_10);
        let git2_config = InMemoryVcs::new(HashMap::new(), HashMap::new(), i64_configs);

        let actual = get_coauthor_configuration(&git2_config);
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

        let actual = get_coauthor_configuration(&git2_config);

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

        let actual = get_coauthor_configuration(&git2_config);

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

        let actual = get_coauthor_configuration(&git2_config);
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
