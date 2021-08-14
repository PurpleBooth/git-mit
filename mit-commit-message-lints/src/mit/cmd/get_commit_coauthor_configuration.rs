use std::{convert::TryInto, time::SystemTimeError};
use std::{
    option::Option,
    result::Result,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::mit::cmd::CONFIG_KEY_EXPIRES;
use crate::mit::VcsError;
use crate::{external::Vcs, mit::Author};

/// Get the co-authors that are currently defined for this vcs config source
///
/// # Errors
///
/// Will fail if reading or writing from the VCS config fails, or it contains
/// data in an incorrect format
pub fn get_commit_coauthor_configuration(
    config: &mut dyn Vcs,
) -> Result<Option<Vec<Author>>, VcsError> {
    let config_value = config.get_i64(CONFIG_KEY_EXPIRES)?;

    match config_value {
        Some(config_value) => {
            let now = now()?;

            if now < Duration::from_secs(config_value.try_into()?) {
                let author_config = get_vcs_authors(config)?;

                Ok(Some(author_config))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

fn now() -> Result<Duration, SystemTimeError> {
    SystemTime::now().duration_since(UNIX_EPOCH)
}

fn get_vcs_authors(config: &dyn Vcs) -> Result<Vec<Author>, VcsError> {
    let co_author_names = get_vcs_coauthor_names(config)?;
    let co_author_emails = get_vcs_coauthor_emails(config)?;

    Ok(co_author_names
        .iter()
        .copied()
        .zip(co_author_emails)
        .filter_map(new_author)
        .collect())
}

fn new_author(parameters: (Option<&str>, Option<&str>)) -> Option<Author> {
    match parameters {
        (Some(name), Some(email)) => Some(Author::new(name, email, None)),
        _ => None,
    }
}

fn get_vcs_coauthor_names(config: &dyn Vcs) -> Result<Vec<Option<&str>>, VcsError> {
    super::vcs::get_vcs_coauthors_config(config, "name")
}

fn get_vcs_coauthor_emails(config: &dyn Vcs) -> Result<Vec<Option<&str>>, VcsError> {
    super::vcs::get_vcs_coauthors_config(config, "email")
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        convert::TryFrom,
        ops::{Add, Sub},
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use pretty_assertions::assert_eq;

    use crate::mit::get_commit_coauthor_configuration;
    use crate::{external::InMemory, mit::Author};

    #[test]
    fn there_is_no_author_config_if_it_has_expired() {
        let now_minus_10 = epoch_with_offset(subtract_100_seconds);
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert(
            super::CONFIG_KEY_EXPIRES.into(),
            format!("{}", now_minus_10),
        );
        let mut vcs = InMemory::new(&mut strings);

        let actual =
            get_commit_coauthor_configuration(&mut vcs).expect("Failed to read VCS config");
        let expected = None;
        assert_eq!(
            expected, actual,
            "Expected the mit config to be {:?}, instead got {:?}",
            expected, actual
        );
    }

    #[test]
    fn there_is_a_config_if_the_config_has_not_expired() {
        let mut strings = BTreeMap::new();
        strings.insert(
            super::CONFIG_KEY_EXPIRES.into(),
            format!("{}", epoch_with_offset(add_100_seconds)),
        );

        let mut vcs = InMemory::new(&mut strings);

        let actual =
            get_commit_coauthor_configuration(&mut vcs).expect("Failed to read VCS config");
        let expected: Option<Vec<Author>> = Some(vec![]);

        assert_eq!(
            expected, actual,
            "Expected the mit config to be {:?}, instead got {:?}",
            expected, actual
        );
    }

    #[test]
    fn we_get_author_config_back_if_there_is_any() {
        let mut buffer = BTreeMap::new();
        buffer.insert(
            super::CONFIG_KEY_EXPIRES.into(),
            format!("{}", epoch_with_offset(add_100_seconds)),
        );
        buffer.insert(
            "mit.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        buffer.insert("mit.author.coauthors.0.name".into(), "Annie Example".into());
        let mut vcs = InMemory::new(&mut buffer);

        let actual =
            get_commit_coauthor_configuration(&mut vcs).expect("Failed to read VCS config");
        let expected = Some(vec![Author::new(
            "Annie Example",
            "annie@example.com",
            None,
        )]);

        assert_eq!(
            expected, actual,
            "Expected the mit config to be {:?}, instead got {:?}",
            expected, actual
        );
    }

    fn add_100_seconds(x: Duration) -> Duration {
        x.add(Duration::from_secs(100))
    }

    fn subtract_100_seconds(x: Duration) -> Duration {
        x.sub(Duration::from_secs(100))
    }

    fn into_seconds(x: Duration) -> u64 {
        x.as_secs()
    }

    #[test]
    fn we_get_multiple_authors_back_if_there_are_multiple() {
        let mut buffer = BTreeMap::new();
        buffer.insert(
            super::CONFIG_KEY_EXPIRES.into(),
            format!("{}", epoch_with_offset(add_100_seconds)),
        );
        buffer.insert(
            "mit.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        buffer.insert("mit.author.coauthors.0.name".into(), "Annie Example".into());
        buffer.insert(
            "mit.author.coauthors.1.email".into(),
            "joe@example.com".into(),
        );
        buffer.insert("mit.author.coauthors.1.name".into(), "Joe Bloggs".into());

        let mut vcs = InMemory::new(&mut buffer);

        let actual =
            get_commit_coauthor_configuration(&mut vcs).expect("Failed to read VCS config");
        let expected = Some(vec![
            Author::new("Annie Example", "annie@example.com", None),
            Author::new("Joe Bloggs", "joe@example.com", None),
        ]);

        assert_eq!(
            expected, actual,
            "Expected the mit config to be {:?}, instead got {:?}",
            expected, actual
        );
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
