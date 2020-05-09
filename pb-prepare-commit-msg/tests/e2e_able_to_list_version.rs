use std::str;

#[test]
fn version_returned_by_long_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output =
        pb_hook_test_helper::run_hook(&working_dir, "pb-prepare-commit-msg", vec!["--version"]);

    let stdout = str::from_utf8(&output.stdout)
        .expect("Failed to convert stdout to a string, is it valid UTF-8?");
    let stderr = str::from_utf8(&output.stderr)
        .expect("Failed to convert stderr to a string, is it valid UTF-8?");

    let expected_prefix = "pb-prepare-commit-msg ";
    assert!(
        stdout.starts_with(expected_prefix),
        "Expected stdout to start with {:?}, instead got stdout: {:?} stderr: {:?}",
        expected_prefix,
        stdout,
        stderr
    );
    assert!(
        stdout.ends_with('\n'),
        "Expected stdout to end with a new line, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );
    assert!(
        &output.status.success(),
        "Expected command to run successfully, instead got {:?}",
        output.status.code()
    );
}

#[test]
fn version_returned_by_short_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-prepare-commit-msg", vec!["-V"]);

    let stderr = str::from_utf8(&output.stderr)
        .expect("Failed to convert stdout to a string, is it valid UTF-8?");
    let stdout = str::from_utf8(&output.stdout)
        .expect("Failed to convert stderr to a string, is it valid UTF-8?");

    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );

    let expected_prefix = "pb-prepare-commit-msg ";
    assert!(
        stdout.starts_with(expected_prefix),
        "Expected stdout to start with {:?}, instead got stdout: {:?} stderr: {:?}",
        expected_prefix,
        stdout,
        stderr
    );
    assert!(
        stdout.ends_with('\n'),
        "Expected stdout to end with a new line, instead got stdout: {:?} stderr: {:?}",
        stdout,
        stderr
    );

    assert!(
        &output.status.success(),
        "Expected command to run successfully, instead got {:?}",
        output.status.code()
    );
}
