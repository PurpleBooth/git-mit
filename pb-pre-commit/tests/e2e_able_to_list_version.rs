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

    let stdout = str::from_utf8(&output.stdout)
        .expect("Failed to convert stdout to a string, is it valid UTF-8?");
    let stderr = str::from_utf8(&output.stderr)
        .expect("Failed to convert stderr to a string, is it valid UTF-8?");

    let expected_prefix = "pb-pre-commit ";
    let status = output.status;

    assert!(
        stdout.starts_with(expected_prefix),
        "Expected stdout to start with {:?}, instead got status: {:?} stdout: {:?} stderr: {:?}",
        expected_prefix,
        status,
        stdout,
        stderr
    );
    assert!(
        stdout.ends_with('\n'),
        "Expected stdout to end with a new line, instead got status: {:?} stdout: {:?} stderr: {:?}",
        status,
        stdout,
        stderr
    );

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead got status: {:?} stdout: {:?} stderr: {:?}",
        status,
        stdout,
        stderr
    );
    assert!(
        &status.success(),
        "Expected command to run successfully, instead got status: {:?} stdout: {:?} stderr: {:?}",
        status,
        stdout,
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

    let stdout = str::from_utf8(&output.stdout)
        .expect("Failed to convert stdout to a string, is it valid UTF-8?");
    let stderr = str::from_utf8(&output.stderr)
        .expect("Failed to convert stderr to a string, is it valid UTF-8?");

    let expected_prefix = "pb-pre-commit ";
    let status = output.status;

    assert!(
        stdout.starts_with(expected_prefix),
        "Expected stdout to start with {:?}, instead got status: {:?} stdout: {:?} stderr: {:?}",
        expected_prefix,
        status,
        stdout,
        stderr
    );
    assert!(
        stdout.ends_with('\n'),
        "Expected stdout to end with a new line, instead got status: {:?} stdout: {:?} stderr: {:?}",
        status,
        stdout,
        stderr
    );

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead got status: {:?} stdout: {:?} stderr: {:?}",
        status,
        stdout,
        stderr
    );
    assert!(
        &status.success(),
        "Expected command to run successfully, instead got status: {:?} stdout: {:?} stderr: {:?}",
        status,
        stdout,
        stderr
    );
}
