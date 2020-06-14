use indoc::indoc;
use std::{
    ops::{Add, Sub},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use mit_hook_test_helper::{assert_output, run_hook, setup_working_dir};

#[test]
fn pre_commit_fails_if_expires_time_has_passed() {
    let working_dir = setup_working_dir();
    mit_hook_test_helper::set_author_expires(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Unix Epoch")
            .sub(Duration::from_secs(100)),
        &working_dir,
    );
    let expected_stderr = indoc!(
        "
        The details of the author of this commit are a bit stale. Can you confirm who's currently \
         coding?

        It's nice to get and give the right credit.

        You can fix this by running `git mit` then the initials of whoever is coding for \
         example:
        git mit bt
        git mit bt se

        "
    );
    let output = run_hook(&working_dir, "mit-pre-commit", vec![]);
    assert_output(&output, "", expected_stderr, false);
}

#[test]
fn pre_commit_does_not_fail_if_time_has_not_passed() {
    let working_dir = setup_working_dir();
    mit_hook_test_helper::set_author_expires(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Unix Epoch")
            .add(Duration::from_secs(100)),
        &working_dir,
    );

    let output = run_hook(&working_dir, "mit-pre-commit", vec![]);

    assert_output(&output, "", "", true);
}
