use pb_hook_test_helper::assert_output;

#[test]
fn help_returned_by_long_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-pre-commit", vec!["--help"]);
    assert_output(
        &output,
        &format!(
            r#"pb-pre-commit {}
Billie Thompson <billie+pb-pre-commit@billiecodes.com>
Run first, before you even type in a commit message. It's used to inspect the snapshot that's about to be committed.

USAGE:
    pb-pre-commit

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
fn help_returned_by_short_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-pre-commit", vec!["-h"]);
    assert_output(
        &output,
        &format!(
            r#"pb-pre-commit {}
Billie Thompson <billie+pb-pre-commit@billiecodes.com>
Run first, before you even type in a commit message. It's used to inspect the snapshot that's about to be committed.

USAGE:
    pb-pre-commit

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
fn short_help_returned_when_a_wrong_message_commands_passed() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-pre-commit", vec!["--banana"]);
    let expected = r#"error: Found argument '--banana' which wasn't expected, or isn't valid in this context

USAGE:
    pb-pre-commit

For more information try --help
"#;

    assert_output(&output, "", expected, false)
}
