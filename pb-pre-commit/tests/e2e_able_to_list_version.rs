use std::str;

#[test]
fn version_returned_by_long_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-pre-commit", vec!["--version"]);

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
        "Expected stdout to end with a new line, instead got status: {:?} stdout: {:?} stderr: \
         {:?}",
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
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-pre-commit", vec!["-V"]);

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
        "Expected stdout to end with a new line, instead got status: {:?} stdout: {:?} stderr: \
         {:?}",
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
