use std::{
    fs,
    io::prelude::*,
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use indoc::indoc;
use pretty_assertions::assert_eq;
use tempfile::NamedTempFile;

use pb_hook_test_helper::{assert_output, set_co_author, setup_working_dir};

#[test]
fn co_author_trailer_should_be_appended() {
    let working_dir = setup_working_dir();
    pb_hook_test_helper::set_author_expires(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Unix Epoch")
            .add(Duration::from_secs(1000)),
        &working_dir,
    );
    set_co_author(&working_dir, "Annie Example", "test@example.com", 0);

    let commit_file = NamedTempFile::new().unwrap();
    writeln!(
        commit_file.as_file(),
        "Lorem Ipsum\n\nIn this commit message I have put a witty message\n"
    )
    .unwrap();

    let actual_output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-prepare-commit-msg",
        vec![&commit_file.path().to_str().unwrap()],
    );

    let actual_commit = fs::read_to_string(commit_file).unwrap();
    let expected_commit = indoc!(
        "
        Lorem Ipsum

        In this commit message I have put a witty message


        Co-authored-by: Annie Example <test@example.com>
        "
    );

    assert_output(&actual_output, "", "", true);

    assert_eq!(
        actual_commit, expected_commit,
        "Expected the commit message to contain {:?}, instead it contained {:?}",
        expected_commit, actual_commit
    );
}

#[test]
fn commit_message_produced_varies_based_on_given_commit_message() {
    let working_dir = setup_working_dir();
    pb_hook_test_helper::set_author_expires(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Unix Epoch")
            .add(Duration::from_secs(1000)),
        &working_dir,
    );
    set_co_author(&working_dir, "Annie Example", "test@example.com", 0);

    let commit_file = NamedTempFile::new().unwrap();
    writeln!(
        commit_file.as_file(),
        "A different mesage\n\nIn this commit message I have put a witty message\n"
    )
    .unwrap();

    let actual_output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-prepare-commit-msg",
        vec![&commit_file.path().to_str().unwrap()],
    );
    let actual_commit = fs::read_to_string(commit_file).unwrap();

    let expected_commit = indoc!(
        "
        A different mesage

        In this commit message I have put a witty message


        Co-authored-by: Annie Example <test@example.com>
        "
    );

    assert_output(&actual_output, "", "", true);
    assert_eq!(
        actual_commit, expected_commit,
        "Expected the commit message to contain {:?}, instead it contained {:?}",
        expected_commit, actual_commit
    );
}

#[test]
fn commit_message_co_author_varies_based_on_message() {
    let working_dir = setup_working_dir();
    pb_hook_test_helper::set_author_expires(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Unix Epoch")
            .add(Duration::from_secs(1000)),
        &working_dir,
    );
    set_co_author(&working_dir, "Joseph Bloggs", "joe@example.com", 0);
    set_co_author(&working_dir, "Annie Example", "annie@example.com", 1);

    let commit_file = NamedTempFile::new().unwrap();

    writeln!(
        commit_file.as_file(),
        "A different mesage\n\nIn this commit message I have put a witty message\n"
    )
    .unwrap();

    let actual_output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-prepare-commit-msg",
        vec![&commit_file.path().to_str().unwrap()],
    );
    let actual_commit = fs::read_to_string(commit_file).unwrap();

    let expected_commit = indoc!(
        "
        A different mesage

        In this commit message I have put a witty message


        Co-authored-by: Joseph Bloggs <joe@example.com>

        Co-authored-by: Annie Example <annie@example.com>
        "
    );

    assert_output(&actual_output, "", "", true);
    assert_eq!(
        actual_commit, expected_commit,
        "Expected the commit message to contain {:?}, instead it contained {:?}",
        expected_commit, actual_commit
    );
}
