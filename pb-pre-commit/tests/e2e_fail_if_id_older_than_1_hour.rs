use std::{
    env,
    ops::{Add, Sub},
    path::PathBuf,
    process::{Command, Output},
    str,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use git2::Repository;
use pretty_assertions::assert_eq;
use tempfile::TempDir;

fn calculate_cargo_toml_path() -> String {
    env::current_exe()
        .unwrap()
        .parent()
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .map(|x| x.join("pb-pre-commit").join("Cargo.toml"))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn run_hook(working_dir: &PathBuf) -> Output {
    Command::new("cargo")
        .current_dir(&working_dir)
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(calculate_cargo_toml_path())
        .output()
        .expect("failed to execute process")
}

fn setup_working_dir() -> PathBuf {
    let temp = TempDir::new()
        .map(|x| x.into_path().join("repository"))
        .expect("Unable to make path");
    Repository::init(&temp).expect("Couldn't create repo");

    temp
}

#[test]
fn pre_commit_fails_if_expires_time_has_passed() {
    let working_dir = setup_working_dir();
    set_author_expires(
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
    set_author_expires(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Unix Epoch")
            .add(Duration::from_secs(100)),
        &working_dir,
    );

    // I know this is weird, default back to git duet
    let expected_stdout =
        "your git duet settings are stale\nupdate them with `git duet` or `git solo`.\n";
    let expected_stderr = "";
    let expect_success = false;
    assert_output(
        &working_dir,
        expected_stdout,
        expected_stderr,
        expect_success,
    );
}

fn set_author_expires(expiration_time: Duration, working_dir: &PathBuf) {
    let now = format!("{}", expiration_time.as_secs());
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("--type")
        .arg("expiry-date")
        .arg("pb.author.expires")
        .arg(now)
        .output()
        .expect("failed to execute process");
}

fn assert_output(
    working_dir: &PathBuf,
    expected_stdout: &str,
    expected_stderr: &str,
    expect_success: bool,
) {
    let output = run_hook(&working_dir);
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
