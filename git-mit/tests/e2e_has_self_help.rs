use indoc::indoc;
use mit_hook_test_helper::assert_output;
use xdg::BaseDirectories;

#[test]
fn help_returned_by_long_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "git-mit", vec!["--help"]);
    let default_config_file = config_file_path();

    let expected_stdout = vec![
        format!("git-mit {}", env!("CARGO_PKG_VERSION")),
        indoc!(
            "
            Billie Thompson <billie+git-mit@billiecodes.com>
            Set author and Co-authored trailer.

            USAGE:
                git-mit [OPTIONS] <initials>...

            ARGS:
                <initials>...    Initials of the author to put in the commit

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            OPTIONS:
                -e, --exec <command>             Execute a command to generate the author configuration, stdout will be captured and
                                                 used instead of the file, if both this and the file is present, this takes
                                                 precedence [env: GIT_MIT_AUTHORS_EXEC=]
                    --completion <completion>    Print completion information for your shell [possible values: bash, fish, zsh,
                                                 elvish]
                -c, --config <file>              Path to a file where author initials, emails and names can be found [env:")
            .into(),

    format!("                                     GIT_MIT_AUTHORS_CONFIG=]  [default: {}]", default_config_file),
        "    -t, --timeout <timeout>          Number of minutes to expire the configuration in [env: GIT_MIT_AUTHORS_TIMEOUT=]".into(),
        "                                     [default: 60]\n".into()
    ]
        .join("\n");

    assert_output(&output, &expected_stdout, "", true)
}

#[test]
fn help_returned_by_short_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "git-mit", vec!["-h"]);
    let default_config_file = config_file_path();

    let expected_stdout = vec![
        format!("git-mit {}", env!("CARGO_PKG_VERSION")),
        indoc!(
            "
            Billie Thompson <billie+git-mit@billiecodes.com>
            Set author and Co-authored trailer.

            USAGE:
                git-mit [OPTIONS] <initials>...

            ARGS:
                <initials>...    Initials of the author to put in the commit

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            OPTIONS:
                -e, --exec <command>             Execute a command to generate the author configuration, stdout will be captured and
                                                 used instead of the file, if both this and the file is present, this takes
                                                 precedence [env: GIT_MIT_AUTHORS_EXEC=]
                    --completion <completion>    Print completion information for your shell [possible values: bash, fish, zsh,
                                                 elvish]
                -c, --config <file>              Path to a file where author initials, emails and names can be found [env:")
            .into(),

        format!("                                     GIT_MIT_AUTHORS_CONFIG=]  [default: {}]", default_config_file),
        "    -t, --timeout <timeout>          Number of minutes to expire the configuration in [env: GIT_MIT_AUTHORS_TIMEOUT=]".into(),
        "                                     [default: 60]\n".into()
    ]
        .join("\n");
    assert_output(&output, &expected_stdout, "", true)
}

fn config_file_path() -> String {
    let cargo_package_name = env!("CARGO_PKG_NAME");
    let add_author_file = |x: BaseDirectories| x.place_config_file("mit.yml").map_err(Box::from);

    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(Box::<dyn std::error::Error>::from)
        .and_then(add_author_file)
        .unwrap()
        .to_string_lossy()
        .to_string()
}

#[test]
fn short_help_returned_when_a_wrong_message_commands_passed() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "git-mit", vec!["--banana"]);
    let expected = indoc!(
        "
        error: Found argument '--banana' which wasn't expected, or isn't valid in this context

        If you tried to supply `--banana` as a PATTERN use `-- --banana`

        USAGE:
            git-mit [OPTIONS] <initials>...

        For more information try --help
        "
    );

    assert_output(&output, "", expected, false)
}
