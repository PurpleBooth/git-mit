use std::process::Command;

use git2::{Config, Repository};
use pretty_assertions::assert_eq;
use std::error::Error;
use tempfile::TempDir;

use pb_commit_message_lints::{
    get_lint_configuration,
    Lints,
    Lints::{DuplicatedTrailers, PivotalTrackerIdMissing},
};
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
};

#[test]
fn with_no_config_return_a_hash_map_default_lints() {
    let config = make_new_config();

    let actual = get_lint_configuration(&config).expect("To be able to get a configuration");

    let expected = vec![DuplicatedTrailers];
    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}

#[test]
fn duplicate_trailer_detection_can_be_disabled() {
    Command::new("git")
        .arg("config")
        .arg("pb.lint.duplicated-trailers")
        .arg("false")
        .output()
        .expect("failed to execute process");

    let mut config = make_new_config();

    config
        .set_bool("pb.lint.duplicated-trailers", false)
        .expect("Failed to disable duplicate trailers?");

    let actual = get_lint_configuration(&config).expect("To be able to get a configuration");

    let expected: Vec<Lints> = vec![];
    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}

#[derive(Debug)]
struct PathError;
impl Error for PathError {}
impl Display for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Path not found")
    }
}

fn make_new_config() -> Config {
    let make_temp_dir = || TempDir::new().map_err(Box::<dyn Error>::from);
    let temp_dir_to_path_buf = |x: TempDir| x.path().to_path_buf();
    let add_repository_to_path = |x: PathBuf| x.join("repository");
    let new_repository_at_path_buf = |x: PathBuf| Repository::init(&x).map_err(Box::from);
    let get_repository_config = |x: Repository| x.config().map_err(Box::from);

    let config: Config = make_temp_dir()
        .map(temp_dir_to_path_buf)
        .map(add_repository_to_path)
        .and_then(new_repository_at_path_buf)
        .and_then(get_repository_config)
        .unwrap();
    config
}

#[test]
fn duplicate_trailer_detection_can_be_explicitly_enabled() {
    let mut config = make_new_config();
    config
        .set_bool("pb.lint.duplicated-trailers", true)
        .expect("Failed to disable duplicate trailers?");

    let actual = get_lint_configuration(&config).expect("To be able to get a configuration");

    let expected: Vec<Lints> = vec![DuplicatedTrailers];
    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}

#[test]
fn pivotal_tracker_id_being_missing_can_be_explicitly_enabled() {
    let mut config = make_new_config();
    config
        .set_bool("pb.lint.pivotal-tracker-id-missing", true)
        .expect("Failed to enable pivotal tracker id?");

    let actual = get_lint_configuration(&config).expect("To be able to get a configuration");

    let expected: Vec<Lints> = vec![DuplicatedTrailers, PivotalTrackerIdMissing];
    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}
