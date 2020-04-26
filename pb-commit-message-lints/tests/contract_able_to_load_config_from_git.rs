use git2::Repository;
use pb_commit_message_lints::Lints::DuplicatedTrailers;
use pb_commit_message_lints::{get_lint_configuration, Lints};
use pretty_assertions::assert_eq;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn with_no_config_return_a_hash_map_default_lints() {
    unset_git_config();
    let config = TempDir::new()
        .map(|x| x.path().join("repository"))
        .map(|x| Repository::init(&x).map(|x| x.config()))
        .unwrap()
        .unwrap()
        .unwrap();
    let actual = get_lint_configuration(&config).expect("To be able to get a configuration");

    let expected = vec![DuplicatedTrailers];
    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}

fn unset_git_config() {
    Command::new("git")
        .arg("config")
        .arg("--local")
        .arg("--unset")
        .arg("pb.message.duplicated-trailers")
        .output()
        .expect("failed to execute process");
}

#[test]
fn duplicate_trailer_detection_can_be_disabled() {
    unset_git_config();
    Command::new("git")
        .arg("config")
        .arg("pb.message.duplicated-trailers")
        .arg("false")
        .output()
        .expect("failed to execute process");

    let mut config = TempDir::new()
        .map(|x| x.path().join("repository"))
        .map(|x| Repository::init(&x).map(|x| x.config()))
        .unwrap()
        .unwrap()
        .unwrap();
    config
        .set_bool("pb.message.duplicated-trailers", false)
        .expect("Failed to disable duplicate trailers?");

    let actual = get_lint_configuration(&config).expect("To be able to get a configuration");

    let expected: Vec<Lints> = vec![];
    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}
#[test]
fn duplicate_trailer_detection_can_be_explicitly_enabled() {
    unset_git_config();

    let mut config = TempDir::new()
        .map(|x| x.path().join("repository"))
        .map(|x| Repository::init(&x).map(|x| x.config()))
        .unwrap()
        .unwrap()
        .unwrap();
    config
        .set_bool("pb.message.duplicated-trailers", true)
        .expect("Failed to disable duplicate trailers?");

    let actual = get_lint_configuration(&config).expect("To be able to get a configuration");

    let expected: Vec<Lints> = vec![DuplicatedTrailers];
    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}
