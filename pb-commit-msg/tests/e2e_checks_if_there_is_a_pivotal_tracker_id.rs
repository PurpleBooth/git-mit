use std::{
    env,
    io::Write,
    path::PathBuf,
    process::{Command, Output},
    str,
};

use git2::Repository;
use tempfile::{NamedTempFile, TempDir};

fn calculate_cargo_toml_path() -> String {
    env::current_exe()
        .unwrap()
        .parent()
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .and_then(std::path::Path::parent)
        .map(|x| x.join("pb-commit-msg").join("Cargo.toml"))
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn run_hook(fake_commit_message: &str, working_dir: &PathBuf) -> Output {
    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", fake_commit_message).unwrap();

    Command::new("cargo")
        .current_dir(&working_dir)
        .arg("run")
        .arg("--quiet")
        .arg("--manifest-path")
        .arg(calculate_cargo_toml_path())
        .arg("--")
        .arg(commit_path.path().to_str().unwrap())
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
fn the_pivotal_tracker_check_does_not_fail_for_a_valid_commit() {
    let input = r#"An example commit

This is an example commit without the pivotal tracker id

[#12345678]
"#;
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.message.pivotal-tracker-id-missing")
        .arg("true")
        .output()
        .expect("failed to execute process");
    let output = run_hook(input, &working_dir);

    let stdout = str::from_utf8(&output.stdout).expect("stdout couldn't be parsed");
    let stderr = str::from_utf8(&output.stderr).expect("stderr couldn't be parsed");

    assert!(
        stdout.is_empty(),
        "Expected stdout to be empty, instead it contained {:?} stderr {:?} status {:?}",
        stdout,
        stderr,
        output.status.code()
    );

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead it contained {:?} stderr {:?} status {:?}",
        stderr,
        stdout,
        output.status.code()
    );

    assert!(
        output.status.success(),
        "Expected status to be a failure, instead it was {:?}  stdout {:?} stderr {:?}",
        &output.status.code(),
        stdout,
        stderr
    );
}

#[test]
fn i_can_enable_the_pivotal_tracker_check() {
    let input = r#"An example commit

This is an example commit without the pivotal tracker id
"#;
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.message.pivotal-tracker-id-missing")
        .arg("true")
        .output()
        .expect("failed to execute process");
    let output = run_hook(input, &working_dir);

    let stdout = str::from_utf8(&output.stdout).expect("stdout couldn't be parsed");
    let stderr = str::from_utf8(&output.stderr).expect("stderr couldn't be parsed");
    let expected_stderr = r#"
An example commit

This is an example commit without the pivotal tracker id


Your commit is missing a Pivotal Tracker Id

Examples:
[Delivers #12345678]
[fixes #12345678]
[finishes #12345678]
[#12345884 #12345678]
[#12345884,#12345678]
[#12345678],[#12345884]
This will address [#12345884]

"#;

    assert!(
        stdout.is_empty(),
        "Expected stdout to be empty, instead it contained {:?} stderr {:?} status {:?}",
        stdout,
        stderr,
        output.status.code()
    );

    assert_eq!(
        stderr,
        expected_stderr,
        "Expected stderr to be contain a nice version out the output, instead it contained {:?} \
         stdout {:?} status {:?}",
        stderr,
        stdout,
        output.status.code()
    );

    assert_eq!(
        output.status.success(),
        false,
        "Expected status to be a failure, instead it was {:?}  stdout {:?} stderr {:?}",
        &output.status.code(),
        stdout,
        stderr
    );
}

#[test]
fn i_can_disable_the_pivotal_tracker_check() {
    let input = r#"An example commit

This is an example commit without the pivotal tracker id
"#;
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.message.pivotal-tracker-id-missing")
        .arg("false")
        .output()
        .expect("failed to execute process");
    let output = run_hook(input, &working_dir);

    let stdout = str::from_utf8(&output.stdout).expect("stdout couldn't be parsed");
    let stderr = str::from_utf8(&output.stderr).expect("stderr couldn't be parsed");

    assert!(
        stdout.is_empty(),
        "Expected stdout to be empty, instead it contained {:?} stderr {:?} status {:?}",
        stdout,
        stderr,
        output.status.code()
    );

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead it contained {:?} stderr {:?} status {:?}",
        stderr,
        stdout,
        output.status.code()
    );

    assert_eq!(
        output.status.success(),
        true,
        "Expected status to be a success, instead it was {:?}  stdout {:?} stderr {:?}",
        &output.status.code(),
        stdout,
        stderr
    );
}
