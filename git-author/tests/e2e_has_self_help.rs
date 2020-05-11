use pb_hook_test_helper::assert_output;

#[test]
fn help_returned_by_long_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-author", vec!["--help"]);
    assert_output(
        &output,
        &format!(
            r#"git-author {}
Billie Thompson <billie+git-author@billiecodes.com>
Set author and Co-authored trailer.

USAGE:
    git-author

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
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-author", vec!["-h"]);
    assert_output(
        &output,
        &format!(
            r#"git-author {}
Billie Thompson <billie+git-author@billiecodes.com>
Set author and Co-authored trailer.

USAGE:
    git-author

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
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-author", vec!["--banana"]);
    let expected = r#"error: Found argument '--banana' which wasn't expected, or isn't valid in this context

USAGE:
    git-author

For more information try --help
"#;

    assert_output(&output, "", expected, false)
}
