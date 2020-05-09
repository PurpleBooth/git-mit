use std::{
    convert::TryFrom,
    ops::{Add, Sub},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use pretty_assertions::assert_eq;

use pb_commit_author::{get_author_configuration, Author};
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
