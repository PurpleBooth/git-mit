use indoc::indoc;
use pb_hook_test_helper::assert_output;

#[test]
fn help_returned_by_long_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output =
        pb_hook_test_helper::run_hook(&working_dir, "pb-prepare-commit-msg", vec!["--help"]);
    let expected_stdout = vec![
        format!("pb-prepare-commit-msg {}", env!("CARGO_PKG_VERSION")),
        indoc!(
            "
            Billie Thompson <billie+pb-prepare-commit-msg@billiecodes.com>
            This hook is invoked by git-commit right after preparing the default log message, and \
             before the editor is started.

            USAGE:
                pb-prepare-commit-msg <commit-message-path> [ARGS]

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            ARGS:
                <commit-message-path>      The name of the file that contains the commit log \
             message
                <commit-message-source>    The commit message, and can be: message (if a -m or -F \
             option was given to git);
                                           template (if a -t option was given or the configuration \
             option commit.template is set
                                           in git); merge (if the commit is a merge or a \
             .git/MERGE_MSG file exists); squash (if
                                           a .git/SQUASH_MSG file exists); or commit
                <commit-sha>               Commit SHA-1 (if a -c, -C or --amend option was given \
             to git).
            "
        )
        .into(),
    ]
    .join("\n");

    assert_output(&output, &expected_stdout, "", true)
}

#[test]
fn help_returned_by_short_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-prepare-commit-msg", vec!["-h"]);
    let expected_stdout = vec![
        format!("pb-prepare-commit-msg {}", env!("CARGO_PKG_VERSION")),
        indoc!(
            "
            Billie Thompson <billie+pb-prepare-commit-msg@billiecodes.com>
            This hook is invoked by git-commit right after preparing the default log message, and \
             before the editor is started.

            USAGE:
                pb-prepare-commit-msg <commit-message-path> [ARGS]

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            ARGS:
                <commit-message-path>      The name of the file that contains the commit log \
             message
                <commit-message-source>    The commit message, and can be: message (if a -m or -F \
             option was given to git);
                                           template (if a -t option was given or the configuration \
             option commit.template is set
                                           in git); merge (if the commit is a merge or a \
             .git/MERGE_MSG file exists); squash (if
                                           a .git/SQUASH_MSG file exists); or commit
                <commit-sha>               Commit SHA-1 (if a -c, -C or --amend option was given \
             to git).
            "
        )
        .into(),
    ]
    .join("\n");

    assert_output(&output, &expected_stdout, "", true)
}

#[test]
fn short_help_returned_when_a_wrong_message_commands_passed() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output =
        pb_hook_test_helper::run_hook(&working_dir, "pb-prepare-commit-msg", vec!["--banana"]);

    let expected_stderr = indoc!(
        "
        error: Found argument '--banana' which wasn't expected, or isn't valid in this context

        USAGE:
            pb-prepare-commit-msg <commit-message-path> [ARGS]

        For more information try --help
        "
    );

    assert_output(&output, "", expected_stderr, false)
}
