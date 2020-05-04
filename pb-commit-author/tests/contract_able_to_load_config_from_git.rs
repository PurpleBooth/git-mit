use git2::Repository;
use pb_commit_author::get_author_configuration;
use pretty_assertions::assert_eq;
use std::{
    convert::TryFrom,
    ops::Sub,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::TempDir;

#[test]
fn there_is_no_author_config_if_it_has_expired() {
    let mut config = TempDir::new()
        .map(TempDir::into_path)
        .map(|x| x.join("repository"))
        .map(Repository::init)
        .expect("Failed to initialise the repository")
        .expect("Failed create temporary directory")
        .config()
        .expect("Failed to get configuration");
    let now_minus_10 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|x| x.sub(Duration::from_secs(10)).as_secs())
        .map(i64::try_from)
        .expect("Failed to get Unix Epoch")
        .expect("Convert epoch to int");
    config
        .set_i64("pb.author.expires", now_minus_10)
        .expect("Failed to set config");
    let actual = get_author_configuration(&config);
    let expected = None;
    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}
