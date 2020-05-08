use std::{
    env,
    fmt,
    ops::{Add, Sub},
    path::PathBuf,
    process::{Command, Output},
    str,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use git2::Repository;
use pretty_assertions::assert_eq;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};
use tempfile::TempDir;

#[derive(Debug)]
struct PathError;
impl Error for PathError {}
impl Display for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Path not found")
    }
}

fn calculate_cargo_toml_path() -> String {
    let boxed_path_error = || Box::from(PathError);
    let parent_directory = |x: PathBuf| x.parent().ok_or_else(boxed_path_error).map(PathBuf::from);
    let bin_root = |x: PathBuf| x.join("pb-pre-commit");
    let cargo_toml = |x: PathBuf| x.join("Cargo.toml");
    let path_buf_to_string = |x: PathBuf| x.to_str().ok_or_else(boxed_path_error).map(String::from);

    env::current_exe()
        .map_err(Box::<dyn Error>::from)
        .and_then(parent_directory)
        .and_then(parent_directory)
        .and_then(parent_directory)
        .and_then(parent_directory)
        .map(bin_root)
        .map(cargo_toml)
        .and_then(path_buf_to_string)
        .unwrap()
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
