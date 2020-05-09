use std::{
    fmt,
    ops::{Add, Sub},
    path::PathBuf,
    str,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use pb_hook_test_helper::setup_working_dir;
use pretty_assertions::assert_eq;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
struct PathError;
impl Error for PathError {}
impl Display for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Path not found")
    }
}

#[test]
fn pre_commit_fails_if_expires_time_has_passed() {
    let working_dir = setup_working_dir();
    pb_hook_test_helper::set_author_expires(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Unix Epoch")
            .sub(Duration::from_secs(100)),
        &working_dir,
    );
    let expected_stdout = "";
    let expected_stderr = r#"
The details of the author of this commit are a bit stale. Can you confirm who's currently coding?

It's nice to get and give the right credit.

You can fix this by running `git author` then the initials of whoever is coding for example:
git author bt
git author bt se
"#;
    let expect_success = false;
    assert_output(
        &working_dir,
        expected_stdout,
        expected_stderr,
        expect_success,
    );
}

#[test]
fn pre_commit_does_not_fail_if_time_has_not_passed() {
    let working_dir = setup_working_dir();
    pb_hook_test_helper::set_author_expires(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Unix Epoch")
            .add(Duration::from_secs(100)),
        &working_dir,
    );

    let expected_stdout = "";
    let expected_stderr = "";
    let expect_success = true;
    assert_output(
        &working_dir,
        expected_stdout,
        expected_stderr,
        expect_success,
    );
}

fn assert_output(
    working_dir: &PathBuf,
    expected_stdout: &str,
    expected_stderr: &str,
    expect_success: bool,
) {
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-pre-commit", vec![]);
    let stdout = str::from_utf8(&output.stdout).expect("stdout couldn't be parsed");
    let stderr = str::from_utf8(&output.stderr).expect("stderr couldn't be parsed");
    assert_eq!(
        stdout,
        expected_stdout,
        "Expected stdout to be {:?}, instead it contained {:?} stderr {:?} status {:?}",
        expected_stdout,
        stdout,
        stderr,
        output.status.code()
    );
    assert_eq!(
        stderr,
        expected_stderr,
        "Expected stderr to {:?}, instead it contained {:?} stderr {:?} status {:?}",
        expected_stderr,
        stderr,
        stdout,
        output.status.code()
    );

    assert_eq!(
        output.status.success(),
        expect_success,
        "Expected status to be {:?}, instead it was {:?}  stdout {:?} stderr {:?}",
        expect_success,
        &output.status.code(),
        stdout,
        stderr
    );
}
