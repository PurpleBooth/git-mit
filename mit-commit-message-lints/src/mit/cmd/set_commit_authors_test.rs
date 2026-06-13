use std::{
    collections::BTreeMap,
    convert::TryFrom,
    error::Error,
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use miette::{miette, Result};

use crate::{
    external::{InMemory, RepoState, Vcs},
    mit::{set_commit_authors, Author},
};

struct FailingVcs;

impl Vcs for FailingVcs {
    fn entries(&self, _glob: Option<&str>) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn get_bool(&self, _name: &str) -> Result<Option<bool>> {
        Ok(None)
    }

    fn get_str(&self, name: &str) -> Result<Option<&str>> {
        if name == "user.signingkey" {
            Ok(Some("existing-key"))
        } else {
            Ok(None)
        }
    }

    fn get_i64(&self, _name: &str) -> Result<Option<i64>> {
        Ok(None)
    }

    fn set_str(&mut self, _name: &str, _value: &str) -> Result<()> {
        Ok(())
    }

    fn set_i64(&mut self, _name: &str, _value: i64) -> Result<()> {
        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<()> {
        if name == "user.signingkey" {
            Err(miette!("simulated remove error"))
        } else {
            Ok(())
        }
    }

    fn state(&self) -> Option<RepoState> {
        None
    }
}

#[test]
fn the_first_initial_becomes_the_author() {
    let mut buffer = BTreeMap::new();

    let mut vcs_config = InMemory::new(&mut buffer);

    let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_hours(1));

    actual.unwrap();
    assert_eq!(
        Some(&"Billie Thompson".to_string()),
        buffer.get("user.name")
    );
    assert_eq!(
        Some(&"billie@example.com".to_string()),
        buffer.get("user.email")
    );
}

#[test]
fn the_first_initial_sets_signing_key_if_it_is_there() {
    let mut str_map = BTreeMap::new();
    let mut vcs_config = InMemory::new(&mut str_map);

    let author = Author::new(
        "Billie Thompson".into(),
        "billie@example.com".into(),
        Some("0A46826A".into()),
    );
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_hours(1));

    actual.unwrap();
    assert_eq!(
        Some(&"0A46826A".to_string()),
        str_map.get("user.signingkey")
    );
}

#[test]
fn the_first_initial_removes_if_it_is_there_and_not_present() {
    let mut buffer = BTreeMap::new();
    buffer.insert("user.signingkey".into(), "0A46826A".into());

    let mut vcs_config = InMemory::new(&mut buffer);

    let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_hours(1));

    actual.unwrap();
    assert_eq!(None, buffer.get("user.signingkey"));
}

#[test]
fn multiple_authors_become_coauthors() {
    let mut buffer = BTreeMap::new();
    let mut vcs_config = InMemory::new(&mut buffer);

    let author_1 = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
    let author_2 = Author::new("Somebody Else".into(), "somebody@example.com".into(), None);
    let author_3 = Author::new("Annie Example".into(), "annie@example.com".into(), None);
    let inputs = vec![&author_1, &author_2, &author_3];

    let actual = set_commit_authors(&mut vcs_config, &inputs, Duration::from_hours(1));

    actual.unwrap();
    assert_eq!(
        Some(&"Billie Thompson".to_string()),
        buffer.get("user.name")
    );
    assert_eq!(
        Some(&"billie@example.com".to_string()),
        buffer.get("user.email")
    );
    assert_eq!(
        Some(&"Somebody Else".to_string()),
        buffer.get("mit.author.coauthors.0.name")
    );
    assert_eq!(
        Some(&"somebody@example.com".to_string()),
        buffer.get("mit.author.coauthors.0.email")
    );
    assert_eq!(
        Some(&"Annie Example".to_string()),
        buffer.get("mit.author.coauthors.1.name")
    );
    assert_eq!(
        Some(&"annie@example.com".to_string()),
        buffer.get("mit.author.coauthors.1.email")
    );
}

#[test]
fn old_co_authors_are_removed() {
    let mut buffer = BTreeMap::new();
    buffer.insert(
        "mit.author.expires".into(),
        format!(
            "{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|x| x.as_secs() + 1000)
                .unwrap()
        ),
    );
    buffer.insert("user.name".into(), "Another Name".into());
    buffer.insert("user.email".into(), "another@example.com".into());
    buffer.insert(
        "mit.author.coauthors.0.name".into(),
        "Different Name".into(),
    );
    buffer.insert(
        "mit.author.coauthors.0.email".into(),
        "different@example.com".into(),
    );
    let mut vcs_config = InMemory::new(&mut buffer);
    let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
    let inputs = vec![&author];

    let actual = set_commit_authors(&mut vcs_config, &inputs, Duration::from_hours(1));

    actual.unwrap();
    assert_eq!(
        Some(&"Billie Thompson".to_string()),
        buffer.get("user.name")
    );
    assert_eq!(
        Some(&"billie@example.com".to_string()),
        buffer.get("user.email")
    );
    assert_eq!(None, buffer.get("mit.author.coauthors.0.name"));
    assert_eq!(None, buffer.get("mit.author.coauthors.0.email"));
}

#[test]
fn sets_the_expiry_time() {
    let mut buffer = BTreeMap::new();
    let mut vcs_config = InMemory::new(&mut buffer);

    let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_hours(1));

    actual.unwrap();

    let sec59min = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|x| x.add(Duration::from_mins(59)))
        .map_err(|x| -> Box<dyn Error> { Box::from(x) })
        .map(|x| x.as_secs())
        .and_then(|x| i64::try_from(x).map_err(Box::from))
        .unwrap();

    let sec61min = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|x| x.add(Duration::from_mins(61)))
        .map_err(|x| -> Box<dyn Error> { Box::from(x) })
        .map(|x| x.as_secs())
        .and_then(|x| i64::try_from(x).map_err(Box::from))
        .unwrap();

    let actual_expire_time: i64 = buffer
        .get("mit.author.expires")
        .and_then(|x| x.parse().ok())
        .expect("Failed to read expire");

    assert!(
        actual_expire_time < sec61min,
        "Expected less than {}, found {}",
        sec61min,
        actual_expire_time
    );
    assert!(
        actual_expire_time > sec59min,
        "Expected more than {} seconds since UNIX EPOCH, found {}",
        sec59min,
        actual_expire_time
    );
}

#[test]
fn propagates_error_when_removing_signing_key_fails() {
    let mut vcs_config = FailingVcs;

    let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_hours(1));

    assert!(actual.is_err());
}

struct ExpiryFailingVcs;

impl Vcs for ExpiryFailingVcs {
    fn entries(&self, _glob: Option<&str>) -> Result<Vec<String>> {
        Ok(vec![])
    }

    fn get_bool(&self, _name: &str) -> Result<Option<bool>> {
        Ok(None)
    }

    fn get_str(&self, _name: &str) -> Result<Option<&str>> {
        Ok(None)
    }

    fn get_i64(&self, _name: &str) -> Result<Option<i64>> {
        Ok(None)
    }

    fn set_str(&mut self, _name: &str, _value: &str) -> Result<()> {
        Ok(())
    }

    fn set_i64(&mut self, _name: &str, _value: i64) -> Result<()> {
        Err(miette!("simulated set_i64 error"))
    }

    fn remove(&mut self, _name: &str) -> Result<()> {
        Ok(())
    }

    fn state(&self) -> Option<RepoState> {
        None
    }
}

#[test]
fn expiry_error_message_mentions_time_not_name() {
    let mut vcs_config = ExpiryFailingVcs;

    let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_hours(1));

    let err = actual.expect_err("expected set_commit_authors to fail with ExpiryFailingVcs");
    let err_msg = format!("{err:#?}");
    assert!(
        err_msg.contains("time") || format!("{err}").contains("time"),
        "Expected the expiry error message to mention 'time', got: {}",
        err_msg
    );
    assert!(
        !format!("{err}").contains("expiry name"),
        "Error message should not say 'expiry name', got: {}",
        err
    );
}
