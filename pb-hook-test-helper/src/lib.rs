use git2::{Config, Repository};
use pretty_assertions::assert_eq;
use std::{
    env,
    error::Error,
    fmt,
    fmt::{Display, Formatter},
    path::PathBuf,
    process::{Command, Output},
    str,
    time::Duration,
};
use tempfile::TempDir;

pub fn run_hook(working_dir: &PathBuf, package: &str, arguments: Vec<&str>) -> Output {
    let toml_path = calculate_cargo_toml_path(package).to_string();
    let mut cargo_arguments = vec!["run", "--quiet", "--manifest-path", &toml_path, "--"];
    cargo_arguments.extend(arguments);

    Command::new("cargo")
        .current_dir(&working_dir)
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

pub fn set_author_expires(expiration_time: Duration, working_dir: &PathBuf) {
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

pub fn make_config() -> Config {
    let add_repository_to_path = |x: PathBuf| x.join("repository");
    let config = TempDir::new()
        .map(TempDir::into_path)
        .map(add_repository_to_path)
        .map(Repository::init)
        .expect("Failed to initialise the repository")
        .expect("Failed create temporary directory")
        .config()
        .expect("Failed to get configuration");
    config
}

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

pub fn setup_working_dir() -> PathBuf {
    let add_repository = |x: PathBuf| x.join("repository");
    let temp = TempDir::new()
        .map(TempDir::into_path)
        .map(add_repository)
        .expect("Unable to make path");
    Repository::init(&temp).expect("Couldn't create repo");

    temp
}
