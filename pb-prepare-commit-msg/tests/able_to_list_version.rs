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
    assert!(&output.status.success());

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.starts_with("pb-prepare-commit-msg "));
    assert!(stdout.ends_with("\n"));

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.is_empty());
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
    assert!(&output.status.success());

    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.starts_with("pb-prepare-commit-msg "));
    assert!(stdout.ends_with("\n"));

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.is_empty());
}
