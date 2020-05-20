use std::process::Command;

use pretty_assertions::assert_eq;

use pb_commit_message_lints::{
    get_lint_configuration,
    Git2VcsConfig,
    Lints,
    Lints::{DuplicatedTrailers, PivotalTrackerIdMissing},
};
use pb_hook_test_helper::make_config;

#[test]
fn with_no_config_return_a_hash_map_default_lints() {
    let config = make_config();

    let git2_config = Git2VcsConfig::new(config);

    let actual = get_lint_configuration(&git2_config).expect("To be able to get a configuration");

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

    let mut config = make_config();

    config
        .set_bool("pb.lint.duplicated-trailers", false)
        .expect("Failed to disable duplicate trailers?");

    let git2_config = Git2VcsConfig::new(config);

    let actual = get_lint_configuration(&git2_config).expect("To be able to get a configuration");
    let expected: Vec<Lints> = vec![];

    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}

#[test]
fn duplicate_trailer_detection_can_be_explicitly_enabled() {
    let mut config = make_config();
    config
        .set_bool("pb.lint.duplicated-trailers", true)
        .expect("Failed to disable duplicate trailers?");

    let git2_config = Git2VcsConfig::new(config);
    let actual = get_lint_configuration(&git2_config).expect("To be able to get a configuration");
    let expected: Vec<Lints> = vec![DuplicatedTrailers];

    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}

#[test]
fn pivotal_tracker_id_being_missing_can_be_explicitly_enabled() {
    let mut config = make_config();
    config
        .set_bool("pb.lint.pivotal-tracker-id-missing", true)
        .expect("Failed to enable pivotal tracker id?");

    let git2_config = Git2VcsConfig::new(config);

    let actual = get_lint_configuration(&git2_config).expect("To be able to get a configuration");
    let expected: Vec<Lints> = vec![DuplicatedTrailers, PivotalTrackerIdMissing];

    assert_eq!(
        expected, actual,
        "Expected the list of lint identifiers to be {:?}, instead got {:?}",
        expected, actual
    )
}
