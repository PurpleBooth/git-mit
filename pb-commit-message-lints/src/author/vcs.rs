use std::{
    ops::Add,
    option::Option,
    result::Result,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::errors::PbCommitMessageLintsError;

use crate::{
    author::entities::Author, errors::PbCommitMessageLintsError::NoAuthorsToSetError,
    external::vcs::Vcs,
};
use std::{convert::TryInto, time::SystemTimeError};

const CONFIG_KEY_EXPIRES: &str = "pb.author.expires";

/// Get the co-authors that are currently defined for this vcs config source
///
/// # Errors
///
/// Will fail if reading or writing from the VCS config fails, or it contains
/// data in an incorrect format
pub fn get_coauthor_configuration(
    config: &mut dyn Vcs,
) -> Result<Option<Vec<Author>>, PbCommitMessageLintsError> {
    match config.get_i64(CONFIG_KEY_EXPIRES) {
        Ok(Some(config_value)) => {
            if now()? < Duration::from_secs(config_value.try_into()?) {
                Ok(Some(get_vcs_authors(config)?))
            } else {
                Ok(None)
            }
        }
        Ok(None) => Ok(None),
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests_able_to_load_config_from_git {
    use std::{
        collections::BTreeMap,
        convert::TryFrom,
        ops::{Add, Sub},
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use pretty_assertions::assert_eq;

    use crate::{
        author::{entities::Author, vcs::get_coauthor_configuration},
        external::vcs::InMemory,
    };

    #[test]
    fn there_is_no_author_config_if_it_has_expired() {
        let now_minus_10 = epoch_with_offset(subtract_10_seconds);
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert("pb.author.expires".into(), format!("{}", now_minus_10));
        let mut vcs = InMemory::new(&mut strings);

        let actual = get_coauthor_configuration(&mut vcs);
        let expected = Ok(None);
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn there_is_a_config_if_the_config_has_not_expired() {
        let mut strings = BTreeMap::new();
        strings.insert(
            "pb.author.expires".into(),
            format!("{}", epoch_with_offset(add_10_seconds)),
        );

        let mut vcs = InMemory::new(&mut strings);

        let actual = get_coauthor_configuration(&mut vcs);
        let expected: Result<Option<Vec<Author>>, _> = Ok(Some(vec![]));

        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn we_get_author_config_back_if_there_is_any() {
        let mut strs = BTreeMap::new();
        strs.insert(
            "pb.author.expires".into(),
            format!("{}", epoch_with_offset(add_10_seconds)),
        );
        strs.insert(
            "pb.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        strs.insert("pb.author.coauthors.0.name".into(), "Annie Example".into());
        let mut vcs = InMemory::new(&mut strs);

        let actual = get_coauthor_configuration(&mut vcs);
        let expected = Ok(Some(vec![Author::new(
            "Annie Example",
            "annie@example.com",
            None,
        )]));

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
        let mut strs = BTreeMap::new();
        strs.insert(
            "pb.author.expires".into(),
            format!("{}", epoch_with_offset(add_10_seconds)),
        );
        strs.insert(
            "pb.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        strs.insert("pb.author.coauthors.0.name".into(), "Annie Example".into());
        strs.insert(
            "pb.author.coauthors.1.email".into(),
            "joe@example.com".into(),
        );
        strs.insert("pb.author.coauthors.1.name".into(), "Joe Bloggs".into());

        let mut vcs = InMemory::new(&mut strs);

        let actual = get_coauthor_configuration(&mut vcs);
        let expected = Ok(Some(vec![
            Author::new("Annie Example", "annie@example.com", None),
            Author::new("Joe Bloggs", "joe@example.com", None),
        ]));

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

fn now() -> Result<Duration, SystemTimeError> {
    SystemTime::now().duration_since(UNIX_EPOCH)
}

fn get_vcs_authors(config: &dyn Vcs) -> Result<Vec<Author>, PbCommitMessageLintsError> {
    let co_author_names = get_vcs_coauthor_names(config)?;
    let co_author_emails = get_vcs_coauthor_emails(config)?;

    Ok(co_author_names
        .iter()
        .zip(co_author_emails)
        .filter_map(new_author)
        .collect())
}

fn new_author(parameters: (&Option<&str>, Option<&str>)) -> Option<Author> {
    match parameters {
        (Some(name), Some(email)) => Some(Author::new(name, email, None)),
        _ => None,
    }
}

fn get_vcs_coauthor_names(
    config: &dyn Vcs,
) -> Result<Vec<Option<&str>>, PbCommitMessageLintsError> {
    get_vcs_coauthors_config(config, "name")
}

fn get_vcs_coauthor_emails(
    config: &dyn Vcs,
) -> Result<Vec<Option<&str>>, PbCommitMessageLintsError> {
    get_vcs_coauthors_config(config, "email")
}

#[allow(clippy::maybe_infinite_iter)]
fn get_vcs_coauthors_config<'a>(
    config: &'a dyn Vcs,
    key: &'a str,
) -> Result<Vec<Option<&'a str>>, PbCommitMessageLintsError> {
    (0..)
        .take_while(|index| has_vcs_coauthor(config, *index))
        .map(|index| get_vcs_coauthor_config(config, key, index))
        .fold(Ok(Vec::<Option<&'a str>>::new()), |acc, item| {
            match (acc, item) {
                (Ok(list), Ok(item)) => Ok(vec![list, vec![item]].concat()),
                (Err(error), Ok(_)) | (Ok(_), Err(error)) | (Err(error), Err(_)) => Err(error),
            }
        })
}

fn get_vcs_coauthor_config<'a>(
    config: &'a dyn Vcs,
    key: &str,
    index: i32,
) -> Result<Option<&'a str>, PbCommitMessageLintsError> {
    config.get_str(&format!("pb.author.coauthors.{}.{}", index, key))
}

fn has_vcs_coauthor(config: &dyn Vcs, index: i32) -> bool {
    let email = get_vcs_coauthor_config(config, "email", index);
    let name = get_vcs_coauthor_config(config, "name", index);

    if let (Ok(Some(_)), Ok(Some(_))) = (name, email) {
        true
    } else {
        false
    }
}

/// # Errors
///
/// This errors if writing to the git authors file fails for some reason. Those
/// reasons will be specific to VCS implementation
pub fn set_authors(
    config: &mut dyn Vcs,
    authors: &[&Author],
    expires_in: Duration,
) -> Result<(), PbCommitMessageLintsError> {
    let (first_author, others) = authors.split_first().ok_or_else(|| NoAuthorsToSetError)?;

    remove_coauthors(config)?;
    set_vcs_user(config, first_author)?;
    set_vcs_coauthors(config, others)?;
    set_vcs_expires_time(config, expires_in)?;

    Ok(())
}

#[cfg(test)]
mod tests_can_set_author_details {
    use std::{
        collections::BTreeMap,
        convert::TryFrom,
        error::Error,
        ops::Add,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use crate::{
        author::{entities::Author, vcs::set_authors},
        external::vcs::InMemory,
    };

    #[test]
    fn the_first_initial_becomes_the_author() {
        let mut strs = BTreeMap::new();

        let mut vcs_config = InMemory::new(&mut strs);

        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let actual = set_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(Some(&"Billie Thompson".to_string()), strs.get("user.name"));
        assert_eq!(
            Some(&"billie@example.com".to_string()),
            strs.get("user.email")
        )
    }

    #[test]
    fn the_first_initial_sets_signing_key_if_it_is_there() {
        let mut str_map = BTreeMap::new();
        let mut vcs_config = InMemory::new(&mut str_map);

        let author = Author::new("Billie Thompson", "billie@example.com", Some("0A46826A"));
        let actual = set_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(
            Some(&"0A46826A".to_string()),
            str_map.get("user.signingkey")
        );
    }

    #[test]
    fn the_first_initial_removes_if_it_is_there_and_not_present() {
        let mut strs = BTreeMap::new();
        strs.insert("user.signingkey".into(), "0A46826A".into());

        let mut vcs_config = InMemory::new(&mut strs);

        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let actual = set_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(None, strs.get("user.signingkey"))
    }

    #[test]
    fn multiple_authors_become_coauthors() {
        let mut strs = BTreeMap::new();
        let mut vcs_config = InMemory::new(&mut strs);

        let author_1 = Author::new("Billie Thompson", "billie@example.com", None);
        let author_2 = Author::new("Somebody Else", "somebody@example.com", None);
        let author_3 = Author::new("Annie Example", "annie@example.com", None);
        let inputs = vec![&author_1, &author_2, &author_3];

        let actual = set_authors(&mut vcs_config, &inputs, Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(Some(&"Billie Thompson".to_string()), strs.get("user.name"));
        assert_eq!(
            Some(&"billie@example.com".to_string()),
            strs.get("user.email")
        );
        assert_eq!(
            Some(&"Somebody Else".to_string()),
            strs.get("pb.author.coauthors.0.name")
        );
        assert_eq!(
            Some(&"somebody@example.com".to_string()),
            strs.get("pb.author.coauthors.0.email")
        );
        assert_eq!(
            Some(&"Annie Example".to_string()),
            strs.get("pb.author.coauthors.1.name")
        );
        assert_eq!(
            Some(&"annie@example.com".to_string()),
            strs.get("pb.author.coauthors.1.email")
        )
    }

    #[test]
    fn old_co_authors_are_removed() {
        let mut strs = BTreeMap::new();
        strs.insert(
            "pb.author.expires".into(),
            format!(
                "{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|x| x.as_secs() + 1000)
                    .unwrap()
            ),
        );
        strs.insert("user.name".into(), "Another Name".into());
        strs.insert("user.email".into(), "another@example.com".into());
        strs.insert("pb.author.coauthors.0.name".into(), "Different Name".into());
        strs.insert(
            "pb.author.coauthors.0.email".into(),
            "different@example.com".into(),
        );
        let mut vcs_config = InMemory::new(&mut strs);
        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let inputs = vec![&author];

        let actual = set_authors(&mut vcs_config, &inputs, Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(Some(&"Billie Thompson".to_string()), strs.get("user.name"));
        assert_eq!(
            Some(&"billie@example.com".to_string()),
            strs.get("user.email")
        );
        assert_eq!(None, strs.get("pb.author.coauthors.0.name"));
        assert_eq!(None, strs.get("pb.author.coauthors.0.email"));
    }

    #[test]
    fn sets_the_expiry_time() {
        let mut strs = BTreeMap::new();
        let mut vcs_config = InMemory::new(&mut strs);

        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let actual = set_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());

        let sec59min = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|x| x.add(Duration::from_secs(60 * 59)))
            .map_err(|x| -> Box<dyn Error> { Box::from(x) })
            .map(|x| x.as_secs())
            .and_then(|x| i64::try_from(x).map_err(Box::from))
            .unwrap();

        let sec61min = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|x| x.add(Duration::from_secs(60 * 61)))
            .map_err(|x| -> Box<dyn Error> { Box::from(x) })
            .map(|x| x.as_secs())
            .and_then(|x| i64::try_from(x).map_err(Box::from))
            .unwrap();

        let actual_expire_time: i64 = strs
            .get("pb.author.expires")
            .and_then(|x| x.parse().ok())
            .expect("Failed to read expire");

        assert_eq!(
            true,
            actual_expire_time < sec61min,
            "Expected less than {}, found {}",
            sec61min,
            actual_expire_time
        );
        assert_eq!(
            true,
            actual_expire_time > sec59min,
            "Expected more than {}, found {}",
            sec59min,
            actual_expire_time
        );
    }
}

fn remove_coauthors(config: &mut dyn Vcs) -> Result<(), PbCommitMessageLintsError> {
    get_defined_vcs_coauthor_keys(config)
        .into_iter()
        .try_for_each(|key| config.remove(&key))
}

#[allow(clippy::maybe_infinite_iter)]
fn get_defined_vcs_coauthor_keys(config: &mut dyn Vcs) -> Vec<String> {
    (0..)
        .take_while(|index| has_vcs_coauthor(config, *index))
        .flat_map(|index| {
            vec![
                format!("pb.author.coauthors.{}.name", index),
                format!("pb.author.coauthors.{}.email", index),
            ]
            .into_iter()
        })
        .map(String::from)
        .collect()
}

fn set_vcs_coauthors(
    config: &mut dyn Vcs,
    authors: &[&Author],
) -> Result<(), PbCommitMessageLintsError> {
    authors
        .iter()
        .enumerate()
        .try_for_each(|(index, author)| set_vcs_coauthor(config, index, author))
}

fn set_vcs_coauthor(
    config: &mut dyn Vcs,
    index: usize,
    author: &Author,
) -> Result<(), PbCommitMessageLintsError> {
    set_vcs_coauthor_name(config, index, author)
        .and_then(|_| set_vcs_coauthor_email(config, index, author))
}

fn set_vcs_coauthor_name(
    config: &mut dyn Vcs,
    index: usize,
    author: &Author,
) -> Result<(), PbCommitMessageLintsError> {
    config.set_str(
        &format!("pb.author.coauthors.{}.name", index),
        &author.name(),
    )?;
    Ok(())
}

fn set_vcs_coauthor_email(
    config: &mut dyn Vcs,
    index: usize,
    author: &Author,
) -> Result<(), PbCommitMessageLintsError> {
    config.set_str(
        &format!("pb.author.coauthors.{}.email", index),
        &author.email(),
    )?;
    Ok(())
}

fn set_vcs_user(config: &mut dyn Vcs, author: &Author) -> Result<(), PbCommitMessageLintsError> {
    config
        .set_str("user.name", &author.name())
        .and_then(|_| config.set_str("user.email", &author.email()))
        .and_then(|_| set_author_signing_key(config, author))
}

fn set_author_signing_key(
    config: &mut dyn Vcs,
    author: &Author,
) -> Result<(), PbCommitMessageLintsError> {
    match author.signingkey() {
        Some(key) => config.set_str("user.signingkey", &key),
        None => config.remove("user.signingkey").or_else(|_| Ok(())),
    }
}

fn set_vcs_expires_time(
    config: &mut dyn Vcs,
    expires_in: Duration,
) -> Result<(), PbCommitMessageLintsError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let expiry_time = now.add(expires_in).as_secs().try_into()?;
    config.set_i64(CONFIG_KEY_EXPIRES, expiry_time)
}
