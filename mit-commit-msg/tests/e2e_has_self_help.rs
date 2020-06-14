use mit_hook_test_helper::assert_output;

#[test]
fn long_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "mit-commit-msg", vec!["--help"]);
    assert_output(
        &output,
        &format!(
            r#"mit-commit-msg {}
Billie Thompson <billie+mit-commit-msg@billiecodes.com>
Validate the commit message that a user has input

USAGE:
    mit-commit-msg <commit-file-path>

ARGS:
    <commit-file-path>    Path to a temporary file that contains the commit message written by the developer

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
"#,
            env!("CARGO_PKG_VERSION")
        ),
        "",
        true,
    )
}

#[test]
fn short_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "mit-commit-msg", vec!["-h"]);
    assert_output(
        &output,
        &format!(
            r#"mit-commit-msg {}
Billie Thompson <billie+mit-commit-msg@billiecodes.com>
Validate the commit message that a user has input

USAGE:
    mit-commit-msg <commit-file-path>

ARGS:
    <commit-file-path>    Path to a temporary file that contains the commit message written by the developer

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
"#,
            env!("CARGO_PKG_VERSION")
        ),
        "",
        true,
    )
}

#[test]
fn invalid_command() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "mit-commit-msg", vec!["--banana"]);
    let expected = r#"error: Found argument '--banana' which wasn't expected, or isn't valid in this context

If you tried to supply `--banana` as a PATTERN use `-- --banana`

USAGE:
    mit-commit-msg <commit-file-path>

For more information try --help
"#;

    assert_output(&output, "", expected, false)
}
