use std::process::Command;
use std::str;

#[test]
fn version_returned_by_long_flag() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("--version")
        .output()
        .expect("failed to execute process");
    assert!(
        &output.status.success(),
        "Expected command to run successfully, instead got {}",
        output.status.code().unwrap()
    );

    let stdout = str::from_utf8(&output.stdout).unwrap();
    let expected_prefix = "pb-commit-msg ";
    assert!(
        stdout.starts_with(expected_prefix),
        "Expected stdout to start with \"{}\", instead got \"{}\"",
        expected_prefix,
        stdout
    );
    assert!(
        stdout.ends_with("\n"),
        "Expected stdout to end with a new line, instead got \"{}\"",
        stdout
    );

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead got \"{}\"",
        stderr
    );
}

#[test]
fn version_returned_by_short_flag() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("-V")
        .output()
        .expect("failed to execute process");

    let stdout = str::from_utf8(&output.stdout).unwrap();
    let expected_prefix = "pb-commit-msg ";
    assert!(
        stdout.starts_with(expected_prefix),
        "Expected stdout to start with \"{}\", instead got \"{}\"",
        expected_prefix,
        stdout
    );
    assert!(
        stdout.ends_with("\n"),
        "Expected stdout to end with a new line, instead got \"{}\"",
        stdout
    );

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead got \"{}\"",
        stderr
    );

    assert!(
        &output.status.success(),
        "Expected command to run successfully, instead got {}",
        output.status.code().unwrap()
    );
}
