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

    let expected = r#"Billie Thompson <billie+pb-prepare-commit-msg@billiecodes.com>
This hook is invoked by git-commit right after preparing the default log message, and before the editor is started.

USAGE:
    pb-prepare-commit-msg <commit-message-path> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <commit-message-path>      The name of the file that contains the commit log message
    <commit-message-source>    The commit message, and can be: message (if a -m or -F option was given to git);
                               template (if a -t option was given or the configuration option commit.template is set
                               in git); merge (if the commit is a merge or a .git/MERGE_MSG file exists); squash (if
                               a .git/SQUASH_MSG file exists); or commit
    <commit-sha>               Commit SHA-1 (if a -c, -C or --amend option was given to git)."#;

    assert!(&stdout.next().unwrap().starts_with("pb-prepare-commit-msg "));

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
    let expected = r#"Billie Thompson <billie+pb-prepare-commit-msg@billiecodes.com>
This hook is invoked by git-commit right after preparing the default log message, and before the editor is started.

USAGE:
    pb-prepare-commit-msg <commit-message-path> [ARGS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <commit-message-path>      The name of the file that contains the commit log message
    <commit-message-source>    The commit message, and can be: message (if a -m or -F option was given to git);
                               template (if a -t option was given or the configuration option commit.template is set
                               in git); merge (if the commit is a merge or a .git/MERGE_MSG file exists); squash (if
                               a .git/SQUASH_MSG file exists); or commit
    <commit-sha>               Commit SHA-1 (if a -c, -C or --amend option was given to git)."#;

    assert!(&stdout.next().unwrap().starts_with("pb-prepare-commit-msg "));

    let actual_stdout = join(stdout, &'\n'.to_string());

    assert_eq!(actual_stdout, expected);
}

#[test]
fn short_help_returned_when_a_wrong_message_commands_passed() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg("--banana")
        .output()
        .expect("failed to execute process");
    assert!(!&output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(stdout.is_empty());

    let stderr = str::from_utf8(&output.stderr).unwrap();
    let expected = r#"error: Found argument '--banana' which wasn't expected, or isn't valid in this context

USAGE:
    pb-prepare-commit-msg <commit-message-path> [ARGS]

For more information try --help
"#;

    assert_eq!(stderr, expected);
}
