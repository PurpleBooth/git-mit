use std::{
    collections::BTreeMap,
    convert::TryFrom,
    error::Error,
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    external::InMemory,
    mit::{set_commit_authors, Author},
};

#[test]
fn the_first_initial_becomes_the_author() {
    let mut buffer = BTreeMap::new();

    let mut vcs_config = InMemory::new(&mut buffer);

    let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

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
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

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
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

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

    let actual = set_commit_authors(&mut vcs_config, &inputs, Duration::from_secs(60 * 60));

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

    let actual = set_commit_authors(&mut vcs_config, &inputs, Duration::from_secs(60 * 60));

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
    let actual = set_commit_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

    actual.unwrap();

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

    let actual_expire_time: i64 = buffer
        .get("mit.author.expires")
        .and_then(|x| x.parse().ok())
        .expect("Failed to read expire");

    assert!(
        actual_expire_time < sec61min,
        "{}",
        "Expected less than {sec61min}, found {actual_expire_time}"
    );
    assert!(
        actual_expire_time > sec59min,
        "{}",
        "Expected more than {sec59min}, found {actual_expire_time}"
    );
}
