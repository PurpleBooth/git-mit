use std::{io::Write, process::Command};

use indoc::indoc;
use pb_hook_test_helper::{assert_output, setup_working_dir};
use tempfile::NamedTempFile;

#[test]
fn valid_commit() {
    let input = indoc!(
        "
        An example commit

        This is an example commit without the pivotal tracker id

        [#12345678]
        "
    );
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.lint.pivotal-tracker-id-missing")
        .arg("true")
        .output()
        .expect("failed to execute process");

    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", input).unwrap();

    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-commit-msg",
        vec![commit_path.path().to_str().unwrap()],
    );

    assert_output(&output, "", "", true)
}

#[test]
fn enabled() {
    let input = indoc!(
        "
        An example commit

        This is an example commit without the pivotal tracker id
        "
    );
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.lint.pivotal-tracker-id-missing")
        .arg("true")
        .output()
        .expect("failed to execute process");

    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", input).unwrap();

    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-commit-msg",
        vec![commit_path.path().to_str().unwrap()],
    );

    let expected_stderr = indoc!(
        "
        An example commit

        This is an example commit without the pivotal tracker id


        ---

        Your commit is missing a Pivotal Tracker Id

        You can fix this by adding the Id in one of the styles below to the commit message
        [Delivers #12345678]
        [fixes #12345678]
        [finishes #12345678]
        [#12345884 #12345678]
        [#12345884,#12345678]
        [#12345678],[#12345884]
        This will address [#12345884]

        "
    );

    assert_output(&output, "", expected_stderr, false)
}

#[test]
fn disabled() {
    let input = indoc!(
        "
        An example commit

        This is an example commit without the pivotal tracker id
        "
    );
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.lint.pivotal-tracker-id-missing")
        .arg("false")
        .output()
        .expect("failed to execute process");

    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", input).unwrap();

    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-commit-msg",
        vec![commit_path.path().to_str().unwrap()],
    );
    assert_output(&output, "", r#""#, true)
}
