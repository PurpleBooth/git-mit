use pretty_assertions::assert_eq;
use std::{
    env,
    path::PathBuf,
    process::{Command, Output},
    str,
    time::{SystemTime, UNIX_EPOCH},
};

use git2::Repository;
use std::{ops::Sub, time::Duration};
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
    let now = format!(
        "{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get Unix Epoch")
            .sub(Duration::from_secs(10))
            .as_secs()
    );
    let working_dir = setup_working_dir();
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

    let output = run_hook(&working_dir);
    let stdout = str::from_utf8(&output.stdout).expect("stdout couldn't be parsed");
    let stderr = str::from_utf8(&output.stderr).expect("stderr couldn't be parsed");

    assert!(
        stdout.is_empty(),
        "Expected stdout to be empty, instead it contained {:?} stderr {:?} status {:?}",
        stdout,
        stderr,
        output.status.code()
    );

    assert_eq!(
        stderr,
        r#"
The details of the author of this commit are a bit stale. Can you confirm who's currently coding?

It's nice to get and give the right credit.

You can fix this by running `git author` then the initials of whoever is coding for example:
git author bt
git author bt se
"#,
        "Expected stderr to be empty, instead it contained {:?} stderr {:?} status {:?}",
        stderr,
        stdout,
        output.status.code()
    );

    assert!(
        !output.status.success(),
        "Expected status to be a failure, instead it was {:?}  stdout {:?} stderr {:?}",
        &output.status.code(),
        stdout,
        stderr
    );
}
