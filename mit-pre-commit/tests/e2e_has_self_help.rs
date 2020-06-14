use indoc::indoc;
use mit_hook_test_helper::assert_output;

#[test]
fn help_returned_by_long_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "mit-pre-commit", vec!["--help"]);
    let expected_stdout = vec![
        format!("mit-pre-commit {}", env!("CARGO_PKG_VERSION")),
        indoc!(
            "
            Billie Thompson <billie+mit-pre-commit@billiecodes.com>
            Run first, before you even type in a commit message. It's used to inspect the snapshot \
             that's about to be committed.

            USAGE:
                mit-pre-commit

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information
            "
        )
        .into(),
    ]
    .join("\n");
    assert_output(&output, &expected_stdout, "", true)
}

#[test]
fn help_returned_by_short_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "mit-pre-commit", vec!["-h"]);

    let expected_stdout = vec![
        format!("mit-pre-commit {}", env!("CARGO_PKG_VERSION")),
        indoc!(
            "
            Billie Thompson <billie+mit-pre-commit@billiecodes.com>
            Run first, before you even type in a commit message. It's used to inspect the snapshot \
             that's about to be committed.

            USAGE:
                mit-pre-commit

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information
            "
        )
        .into(),
    ]
    .join("\n");
    assert_output(&output, &expected_stdout, "", true)
}

#[test]
fn short_help_returned_when_a_wrong_message_commands_passed() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "mit-pre-commit", vec!["--banana"]);

    let expected_stderr = indoc!(
        "
        error: Found argument '--banana' which wasn't expected, or isn't valid in this context

        USAGE:
            mit-pre-commit

        For more information try --help
        "
    );

    assert_output(&output, "", &expected_stderr, false)
}
