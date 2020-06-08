use indoc::indoc;
use pb_hook_test_helper::assert_output;
use xdg::BaseDirectories;

#[test]
fn help_returned_by_long_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-authors", vec!["--help"]);
    let default_config_file = config_file_path();

    let expected_stdout = vec![
        format!("git-authors {}", env!("CARGO_PKG_VERSION")),
        indoc!(
            "
            Billie Thompson <billie+git-author@billiecodes.com>
            Set author and Co-authored trailer.

            USAGE:
                git-authors [OPTIONS] <initials>...

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            OPTIONS:
                -e, --exec <command>       Execute a command to generate the author configuration, stdout will be captured and used
                                           instead of the file, if both this and the file is present, this takes precedence [env:
                                           GIT_AUTHORS_EXEC=]
                -c, --config <file>        Path to a file where authors initials, emails and names can be found [env:")
            .into(),

    format!("                               GIT_AUTHORS_CONFIG=]  [default: {}]", default_config_file),
    indoc!("
                -t, --timeout <timeout>    Number of minutes to expire the configuration in [env: GIT_AUTHORS_TIMEOUT=]  [default:
                                           60]

            ARGS:
                <initials>...    Initials of the authors to put in the commit
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
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-authors", vec!["-h"]);
    let default_config_file = config_file_path();

    let expected_stdout = vec![
        format!("git-authors {}", env!("CARGO_PKG_VERSION")),
        indoc!(
            "
            Billie Thompson <billie+git-author@billiecodes.com>
            Set author and Co-authored trailer.

            USAGE:
                git-authors [OPTIONS] <initials>...

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            OPTIONS:
                -e, --exec <command>       Execute a command to generate the author configuration, stdout will be captured and used
                                           instead of the file, if both this and the file is present, this takes precedence [env:
                                           GIT_AUTHORS_EXEC=]
                -c, --config <file>        Path to a file where authors initials, emails and names can be found [env:")
            .into(),

        format!("                               GIT_AUTHORS_CONFIG=]  [default: {}]", default_config_file),
        indoc!("
                -t, --timeout <timeout>    Number of minutes to expire the configuration in [env: GIT_AUTHORS_TIMEOUT=]  [default:
                                           60]

            ARGS:
                <initials>...    Initials of the authors to put in the commit
            "
        )
            .into(),
    ]
        .join("\n");
    assert_output(&output, &expected_stdout, "", true)
}

fn config_file_path() -> String {
    let cargo_package_name = env!("CARGO_PKG_NAME");
    let add_author_file =
        |x: BaseDirectories| x.place_config_file("authors.yml").map_err(Box::from);

    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(Box::<dyn std::error::Error>::from)
        .and_then(add_author_file)
        .unwrap()
        .to_string_lossy()
        .to_string()
}

#[test]
fn short_help_returned_when_a_wrong_message_commands_passed() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-authors", vec!["--banana"]);
    let expected = indoc!(
        "
        error: Found argument '--banana' which wasn't expected, or isn't valid in this context

        USAGE:
            git-authors [OPTIONS] <initials>...

        For more information try --help
        "
    );

    assert_output(&output, "", expected, false)
}
