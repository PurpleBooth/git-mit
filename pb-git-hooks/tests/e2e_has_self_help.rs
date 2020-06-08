use pb_hook_test_helper::assert_output;

#[test]
fn help_printed_when_no_arguments_passed() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-git-hooks", vec![]);
    let expected = &format!(
        r#"pb-git-hooks {}
Billie Thompson <billie+pb-git-hooks@billiecodes.com>
A command for enabling and disabling git lints

USAGE:
    pb-git-hooks [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --scope <scope>     [default: local]  [possible values: local, global]

SUBCOMMANDS:
    authors    Manage author configuration
    help       Prints this message or the help of the given subcommand(s)
    lint       Manage active lints
"#,
        env!("CARGO_PKG_VERSION")
    );

    assert_output(&output, "", expected, false)
}

#[test]
fn lint_alone_provides_help() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-git-hooks", vec!["lint"]);
    let expected = format!(
        "pb-git-hooks-lint {}
Manage active lints

USAGE:
    pb-git-hooks lint <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    available    List the available lints
    disable      Disable a lint
    enable       Enable a lint
    enabled      List the enabled lints
    help         Prints this message or the help of the given subcommand(s)
    status       Get status of a lint
",
        ""
    );

    assert_output(&output, "", &expected, false)
}
#[test]
fn authors_alone_provides_help() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-git-hooks", vec!["authors"]);
    let expected = format!(
        "pb-git-hooks-authors {}
Manage author configuration

USAGE:
    pb-git-hooks authors <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    example    Print example author yaml file
    help       Prints this message or the help of the given subcommand(s)
",
        ""
    );

    assert_output(&output, "", &expected, false)
}
