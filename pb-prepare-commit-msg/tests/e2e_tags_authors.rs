use std::{
    fmt,
    fs,
    io::prelude::*,
    ops::Add,
    path::PathBuf,
    process::{Command, Output},
    str,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use pretty_assertions::assert_eq;

use pb_hook_test_helper::setup_working_dir;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};
use tempfile::NamedTempFile;

#[derive(Debug)]
struct PathError;
impl Error for PathError {}
impl Display for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Path not found")
    }
}

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

    let commit_message_file = NamedTempFile::new().unwrap();
    writeln!(
        commit_message_file.as_file(),
        r#"Lorem Ipsum

In this commit message I have put a witty message"#
    )
    .unwrap();

    let actual_output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-prepare-commit-msg",
        vec![&commit_message_file.path().to_str().unwrap()],
    );

    let actual_commit_message = fs::read_to_string(commit_message_file).unwrap();

    let expected_stdout = "";
    let expected_stderr = r#""#;
    let expect_success = true;
    let expected_commit_message = r#"Lorem Ipsum

In this commit message I have put a witty message

Co-authored-by: Annie Example <test@example.com>
"#;

    assert_output(
        actual_output,
        expected_stdout,
        expected_stderr,
        expect_success,
    );
    assert_eq!(
        actual_commit_message, expected_commit_message,
        "Expected the commit message to contain {:?}, instead it contained {:?}",
        expected_commit_message, actual_commit_message
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

    let commit_message_file = NamedTempFile::new().unwrap();
    writeln!(
        commit_message_file.as_file(),
        r#"A different mesage

In this commit message I have put a witty message"#
    )
    .unwrap();

    let actual_output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-prepare-commit-msg",
        vec![&commit_message_file.path().to_str().unwrap()],
    );
    let actual_commit_message = fs::read_to_string(commit_message_file).unwrap();

    let expected_stdout = "";
    let expected_stderr = r#""#;
    let expect_success = true;
    let expected_commit_message = r#"A different mesage

In this commit message I have put a witty message

Co-authored-by: Annie Example <test@example.com>
"#;

    assert_output(
        actual_output,
        expected_stdout,
        expected_stderr,
        expect_success,
    );
    assert_eq!(
        actual_commit_message, expected_commit_message,
        "Expected the commit message to contain {:?}, instead it contained {:?}",
        expected_commit_message, actual_commit_message
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

    let commit_message_file = NamedTempFile::new().unwrap();
    writeln!(
        commit_message_file.as_file(),
        r#"A different mesage

In this commit message I have put a witty message"#
    )
    .unwrap();

    let actual_output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-prepare-commit-msg",
        vec![&commit_message_file.path().to_str().unwrap()],
    );
    let actual_commit_message = fs::read_to_string(commit_message_file).unwrap();

    let expected_stdout = "";
    let expected_stderr = r#""#;
    let expect_success = true;
    let expected_commit_message = r#"A different mesage

In this commit message I have put a witty message

Co-authored-by: Joseph Bloggs <joe@example.com>
Co-authored-by: Annie Example <annie@example.com>
"#;

    assert_output(
        actual_output,
        expected_stdout,
        expected_stderr,
        expect_success,
    );
    assert_eq!(
        actual_commit_message, expected_commit_message,
        "Expected the commit message to contain {:?}, instead it contained {:?}",
        expected_commit_message, actual_commit_message
    );
}

fn set_co_author(working_dir: &PathBuf, author_name: &str, author_email: &str, index: i64) {
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg(format!("pb.author.coauthors.{}.name", index))
        .arg(author_name)
        .output()
        .expect("failed to execute process");
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg(format!("pb.author.coauthors.{}.email", index))
        .arg(author_email)
        .output()
        .expect("failed to execute process");
}

fn assert_output(
    output: Output,
    expected_stdout: &str,
    expected_stderr: &str,
    expect_success: bool,
) {
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
