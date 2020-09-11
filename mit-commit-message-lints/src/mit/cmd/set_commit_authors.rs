use std::convert::TryInto;
use std::{
    ops::Add,
    result::Result,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::mit::cmd::vcs::has_vcs_coauthor;
use crate::mit::cmd::CONFIG_KEY_EXPIRES;
use crate::mit::VcsError;
use crate::{external::Vcs, mit::Author};

/// # Errors
///
/// This errors if writing to the git mit file fails for some reason. Those
/// reasons will be specific to VCS implementation
pub fn set_commit_authors(
    config: &mut dyn Vcs,
    authors: &[&Author],
    expires_in: Duration,
) -> Result<(), VcsError> {
    let (first_author, others) = authors.split_first().ok_or(VcsError::NoAuthorsToSet)?;

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

    use crate::mit::set_commit_authors;
    use crate::{external::InMemory, mit::Author};

    #[test]
    fn the_first_initial_becomes_the_author() {
        let mut strs = BTreeMap::new();

        let mut vcs_config = InMemory::new(&mut strs);

        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

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
        let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

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
        let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

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

        let actual = set_commit_authors(&mut vcs_config, &inputs, Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(Some(&"Billie Thompson".to_string()), strs.get("user.name"));
        assert_eq!(
            Some(&"billie@example.com".to_string()),
            strs.get("user.email")
        );
        assert_eq!(
            Some(&"Somebody Else".to_string()),
            strs.get("mit.author.coauthors.0.name")
        );
        assert_eq!(
            Some(&"somebody@example.com".to_string()),
            strs.get("mit.author.coauthors.0.email")
        );
        assert_eq!(
            Some(&"Annie Example".to_string()),
            strs.get("mit.author.coauthors.1.name")
        );
        assert_eq!(
            Some(&"annie@example.com".to_string()),
            strs.get("mit.author.coauthors.1.email")
        )
    }

    #[test]
    fn old_co_authors_are_removed() {
        let mut strs = BTreeMap::new();
        strs.insert(
            "mit.author.expires".into(),
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
        strs.insert(
            "mit.author.coauthors.0.name".into(),
            "Different Name".into(),
        );
        strs.insert(
            "mit.author.coauthors.0.email".into(),
            "different@example.com".into(),
        );
        let mut vcs_config = InMemory::new(&mut strs);
        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let inputs = vec![&author];

        let actual = set_commit_authors(&mut vcs_config, &inputs, Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(Some(&"Billie Thompson".to_string()), strs.get("user.name"));
        assert_eq!(
            Some(&"billie@example.com".to_string()),
            strs.get("user.email")
        );
        assert_eq!(None, strs.get("mit.author.coauthors.0.name"));
        assert_eq!(None, strs.get("mit.author.coauthors.0.email"));
    }

    #[test]
    fn sets_the_expiry_time() {
        let mut strs = BTreeMap::new();
        let mut vcs_config = InMemory::new(&mut strs);

        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

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
            .get("mit.author.expires")
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

fn remove_coauthors(config: &mut dyn Vcs) -> Result<(), VcsError> {
    get_defined_vcs_coauthor_keys(config)
        .into_iter()
        .try_for_each(|key| config.remove(&key))?;

    Ok(())
}

#[allow(clippy::maybe_infinite_iter)]
fn get_defined_vcs_coauthor_keys(config: &mut dyn Vcs) -> Vec<String> {
    (0..)
        .take_while(|index| has_vcs_coauthor(config, *index))
        .flat_map(|index| {
            vec![
                format!("mit.author.coauthors.{}.name", index),
                format!("mit.author.coauthors.{}.email", index),
            ]
            .into_iter()
        })
        .map(String::from)
        .collect()
}

fn set_vcs_coauthors(config: &mut dyn Vcs, authors: &[&Author]) -> Result<(), VcsError> {
    authors
        .iter()
        .enumerate()
        .try_for_each(|(index, author)| set_vcs_coauthor(config, index, author))
}

fn set_vcs_coauthor(config: &mut dyn Vcs, index: usize, author: &Author) -> Result<(), VcsError> {
    set_vcs_coauthor_name(config, index, author)?;
    set_vcs_coauthor_email(config, index, author)?;

    Ok(())
}

fn set_vcs_coauthor_name(
    config: &mut dyn Vcs,
    index: usize,
    author: &Author,
) -> Result<(), VcsError> {
    config.set_str(
        &format!("mit.author.coauthors.{}.name", index),
        &author.name(),
    )?;
    Ok(())
}

fn set_vcs_coauthor_email(
    config: &mut dyn Vcs,
    index: usize,
    author: &Author,
) -> Result<(), VcsError> {
    config.set_str(
        &format!("mit.author.coauthors.{}.email", index),
        &author.email(),
    )?;
    Ok(())
}

fn set_vcs_user(config: &mut dyn Vcs, author: &Author) -> Result<(), VcsError> {
    config.set_str("user.name", &author.name())?;
    config.set_str("user.email", &author.email())?;
    set_author_signing_key(config, author)?;

    Ok(())
}

fn set_author_signing_key(config: &mut dyn Vcs, author: &Author) -> Result<(), VcsError> {
    match author.signingkey() {
        Some(key) => config
            .set_str("user.signingkey", &key)
            .map_err(VcsError::from),
        None => config.remove("user.signingkey").or_else(|_| Ok(())),
    }
}

fn set_vcs_expires_time(config: &mut dyn Vcs, expires_in: Duration) -> Result<(), VcsError> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let expiry_time = now.add(expires_in).as_secs().try_into()?;
    config
        .set_i64(CONFIG_KEY_EXPIRES, expiry_time)
        .map_err(VcsError::from)
}
