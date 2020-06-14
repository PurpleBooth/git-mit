use indoc::indoc;
use mit_hook_test_helper::assert_output;

#[test]
fn help_printed_when_no_arguments_passed() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "git-mit-config", vec![]);
    let expected = vec![
        format!("git-mit-config {}", env!("CARGO_PKG_VERSION")),
        indoc!(
            "
            Billie Thompson <billie+git-mit-config@billiecodes.com>
            A command for enabling and disabling git lints

            USAGE:
                git-mit-config [OPTIONS] <SUBCOMMAND>

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            OPTIONS:
                -s, --scope <scope>     [default: local]  [possible values: local, global]

            SUBCOMMANDS:
                completion    Print completion information
                help          Prints this message or the help of the given subcommand(s)
                lint          Manage active lints
                mit           Manage author configuration
            "
        )
        .into(),
    ]
    .join("\n");

    assert_output(&output, "", &expected, false)
}

#[test]
fn lint_alone_provides_help() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "git-mit-config", vec!["lint"]);

    let expected = vec![
        format!("git-mit-config-lint {}", ""),
        indoc!(
            "
            Manage active lints

            USAGE:
                git-mit-config lint <SUBCOMMAND>

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
            "
        )
        .into(),
    ]
    .join("\n");

    assert_output(&output, "", &expected, false)
}
#[test]
fn authors_alone_provides_help() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "git-mit-config", vec!["mit"]);

    let expected = vec![
        format!("git-mit-config-mit {}", ""),
        indoc!(
            "
            Manage author configuration

            USAGE:
                git-mit-config mit <SUBCOMMAND>

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            SUBCOMMANDS:
                example    Print example author yaml file
                help       Prints this message or the help of the given subcommand(s)
            "
        )
        .into(),
    ]
    .join("\n");

    assert_output(&output, "", &expected, false)
}
