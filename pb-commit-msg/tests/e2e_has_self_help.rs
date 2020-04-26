use std::process::Command;
use std::str;

use itertools::join;
use pretty_assertions::assert_eq;

#[test]
fn help_returned_by_long_flag() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("--help")
        .output()
        .expect("failed to execute process");
    assert!(&output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.is_empty());

    let mut stdout = str::from_utf8(&output.stdout).unwrap().lines();
    let expected = r#"Billie Thompson <billie+pb-commit-msg@billiecodes.com>
Validate the commit message that a user has input

USAGE:
    pb-commit-msg <commit-file-path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <commit-file-path>    Path to a temporary file that contains the commit message written by the developer"#;

    assert!(&stdout.next().unwrap().starts_with("pb-commit-msg "));

    let actual_stdout = join(stdout, &'\n'.to_string());

    assert_eq!(actual_stdout, expected);
}

#[test]
fn help_returned_by_short_flag() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("-h")
        .output()
        .expect("failed to execute process");
    assert!(&output.status.success());
    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(stderr.is_empty());

    let mut stdout = str::from_utf8(&output.stdout).unwrap().lines();
    let expected = r#"Billie Thompson <billie+pb-commit-msg@billiecodes.com>
Validate the commit message that a user has input

USAGE:
    pb-commit-msg <commit-file-path>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <commit-file-path>    Path to a temporary file that contains the commit message written by the developer"#;

    assert!(&stdout.next().unwrap().starts_with("pb-commit-msg "));

    let actual_stdout = join(stdout, &'\n'.to_string());

    assert_eq!(actual_stdout, expected);
}

#[test]
fn short_help_returned_when_a_wrong_message_commands_passed() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("-q")
        .arg("-w")
        .arg("-e")
        .arg("-r")
        .arg("-t")
        .arg("-y")
        .arg("-i")
        .arg("-o")
        .arg("-u")
        .arg("-p")
        .output()
        .expect("failed to execute process");
    assert!(!&output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.is_empty());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    let expected = r#"error: Found argument '-q' which wasn't expected, or isn't valid in this context

USAGE:
    pb-commit-msg <commit-file-path>

For more information try --help
"#;

    assert_eq!(stderr, expected);
}
