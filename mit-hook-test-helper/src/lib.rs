//! Tools for making tests less filled with boilerplate

#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

use std::{
    env,
    error::Error,
    fmt,
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
    process::{Command, Output},
    str,
    time::Duration,
};

use git2::{Config, Repository};
use tempfile::TempDir;

/// Run a specific hook binary
///
/// # Panics
///
/// If the cargo command fails to run or for some reason running the hook fails
#[must_use]
pub fn run_hook(working_dir: &Path, package: &str, arguments: Vec<&str>) -> Output {
    let toml_path = calculate_cargo_toml_path(package);
    let mut cargo_arguments = vec![
        "run",
        "--locked",
        "--quiet",
        "--manifest-path",
        &toml_path,
        "--",
    ];
    cargo_arguments.extend(arguments);

    Command::new("cargo")
        .current_dir(working_dir)
        .args(cargo_arguments)
        .output()
        .expect("failed to execute process")
}

#[derive(Debug)]
struct PathError;

impl Error for PathError {}

impl Display for PathError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Path not found")
    }
}
/// set the co-authors via the git binary
///
/// # Panics
///
/// If the git binary fails to run for some reason, or it fails to set the
/// configuration due to a readonly filesystem or similar.
pub fn set_co_author(working_dir: &Path, author_name: &str, author_email: &str, index: i64) {
    Command::new("git")
        .current_dir(working_dir)
        .arg("config")
        .arg("--local")
        .arg(format!("mit.author.coauthors.{index}.name"))
        .arg(author_name)
        .output()
        .expect("failed to execute process");
    Command::new("git")
        .current_dir(working_dir)
        .arg("config")
        .arg("--local")
        .arg(format!("mit.author.coauthors.{index}.email"))
        .arg(author_email)
        .output()
        .expect("failed to execute process");
}

/// Set the authors expires time via the git binary
///
/// # Panics
///
/// If the git binary fails to execute, for example if it was not found or was
/// broken in some way
pub fn set_author_expires(expiration_time: Duration, working_dir: &Path) {
    let now = format!("{}", expiration_time.as_secs());
    Command::new("git")
        .current_dir(working_dir)
        .arg("config")
        .arg("--local")
        .arg("--type")
        .arg("expiry-date")
        .arg("mit.author.expires")
        .arg(now)
        .output()
        .expect("failed to execute process");
}

/// # Panics
///
/// if it can't calculate the path to the cargo toml
#[must_use]
pub fn calculate_cargo_toml_path(package: &str) -> String {
    let boxed_path_error = || Box::from(PathError);
    let parent_directory = |x: PathBuf| x.parent().ok_or_else(boxed_path_error).map(PathBuf::from);
    let bin_root = |x: PathBuf| x.join(package);
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

/// Make a config object on a repo in a temporary directory
///
/// # Panics
///
/// Panics on failure to create a temporary directory, to initialise a git repo
/// (for example if the filesystem was readonly) or to get the configuration if
/// the configuration was malformed
#[must_use]
pub fn make_config() -> Config {
    let add_repository_to_path = |x: PathBuf| x.join("repository");
    TempDir::new()
        .map(TempDir::into_path)
        .map(add_repository_to_path)
        .map(Repository::init)
        .expect("Failed to initialise the repository")
        .expect("Failed create temporary directory")
        .config()
        .expect("Failed to get configuration")
}

/// # Panics
///
/// Panics on failed test
pub fn assert_output(
    output: &Output,
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

/// Get working directory
///
/// This is a new temporary directory with a git repo in it
///
/// # Panics
///
/// Panics on failed creation of temporary directory, or on initialising git
/// repo (for example if filesystem is read only)
#[must_use]
pub fn setup_working_dir() -> PathBuf {
    let add_repository = |x: PathBuf| x.join("repository");
    let temp = TempDir::new()
        .map(TempDir::into_path)
        .map(add_repository)
        .expect("Unable to make path");
    Repository::init(&temp).expect("Couldn't create repo");

    temp
}
