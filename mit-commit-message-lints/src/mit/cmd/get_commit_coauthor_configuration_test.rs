use std::{
    collections::BTreeMap,
    convert::TryFrom,
    ops::{Add, Sub},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use time::OffsetDateTime;

use crate::{
    external::InMemory,
    mit::{get_commit_coauthor_configuration, Author, AuthorState},
};

#[test]
fn there_is_no_author_config_if_it_has_expired() {
    let now_minus_10 = epoch_with_offset(subtract_100_seconds);
    let mut strings: BTreeMap<String, String> = BTreeMap::new();
    strings.insert(super::CONFIG_KEY_EXPIRES.into(), format!("{now_minus_10}"));
    let vcs = InMemory::new(&mut strings);

    let actual = get_commit_coauthor_configuration(&vcs).expect("Failed to read VCS config");
    let expected = AuthorState::Timeout(OffsetDateTime::from_unix_timestamp(now_minus_10).unwrap());
    assert_eq!(
        expected, actual,
        "Expected the mit config to be {expected:?}, instead got {actual:?}"
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

    let actual = get_commit_coauthor_configuration(&vcs).expect("Failed to read VCS config");
    let expected: AuthorState<Vec<Author<'_>>> = AuthorState::Some(vec![]);

    assert_eq!(
        expected, actual,
        "Expected the mit config to be {expected:?}, instead got {actual:?}"
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
    let vcs = InMemory::new(&mut buffer);

    let actual = get_commit_coauthor_configuration(&vcs).expect("Failed to read VCS config");
    let expected = AuthorState::Some(vec![Author::new(
        "Annie Example".into(),
        "annie@example.com".into(),
        None,
    )]);

    assert_eq!(
        expected, actual,
        "Expected the mit config to be {expected:?}, instead got {actual:?}"
    );
}

fn add_100_seconds(x: Duration) -> Duration {
    x.add(Duration::from_secs(100))
}

fn subtract_100_seconds(x: Duration) -> Duration {
    x.sub(Duration::from_secs(100))
}

const fn into_seconds(x: Duration) -> u64 {
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

    let vcs = InMemory::new(&mut buffer);

    let actual = get_commit_coauthor_configuration(&vcs).expect("Failed to read VCS config");
    let expected = AuthorState::Some(vec![
        Author::new("Annie Example".into(), "annie@example.com".into(), None),
        Author::new("Joe Bloggs".into(), "joe@example.com".into(), None),
    ]);

    assert_eq!(
        expected, actual,
        "Expected the mit config to be {expected:?}, instead got {actual:?}"
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
