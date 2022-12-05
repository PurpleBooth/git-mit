use std::{
    collections::BTreeMap,
    convert::TryFrom,
    ops::{Add, Sub},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    external::InMemory,
    relates::{get_relate_to_configuration, RelateTo},
};

#[test]
fn there_is_no_relate_config_if_it_has_expired() {
    let now_minus_10 = epoch_with_offset(subtract_10_seconds);
    let mut strings: BTreeMap<String, String> = BTreeMap::new();
    strings.insert("mit.relate.expires".into(), format!("{now_minus_10}"));
    let mut vcs = InMemory::new(&mut strings);

    let actual = get_relate_to_configuration(&mut vcs).expect("Failed to read VCS config");
    let expected = None;
    assert_eq!(
        expected, actual,
        "Expected the relate config to be {expected:?}, instead got {actual:?}"
    );
}

#[test]
fn we_get_relate_to_config_back_if_there_is_any() {
    let mut buffer = BTreeMap::new();
    buffer.insert(
        "mit.relate.expires".into(),
        format!("{}", epoch_with_offset(add_10_seconds)),
    );
    buffer.insert("mit.relate.to".into(), "[#12345678]".into());
    let mut vcs = InMemory::new(&mut buffer);

    let actual = get_relate_to_configuration(&mut vcs).expect("Failed to read VCS config");
    let expected = Some(RelateTo::from("[#12345678]"));

    assert_eq!(
        expected, actual,
        "Expected the relate config to be {expected:?}, instead got {actual:?}"
    );
}

fn add_10_seconds(x: Duration) -> Duration {
    x.add(Duration::from_secs(10))
}

fn subtract_10_seconds(x: Duration) -> Duration {
    x.sub(Duration::from_secs(10))
}

const fn into_seconds(x: Duration) -> u64 {
    x.as_secs()
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
