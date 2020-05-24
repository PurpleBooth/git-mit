use xdg::BaseDirectories;

use pb_hook_test_helper::assert_output;

#[test]
fn help_returned_by_long_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-authors", vec!["--help"]);
    let default_config_file = config_file_path();
    assert_output(
        &output,
        &format!(
            r#"git-authors {}
Billie Thompson <billie+git-author@billiecodes.com>
Set author and Co-authored trailer.

USAGE:
    git-authors [OPTIONS] <author-initial>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <author-file-path>    Initials of the authors to put in the commit [env: GIT_AUTHORS_AUTHOR_FILE_PATH=]
                                       [default: {}]
    -t, --timeout <timeout>            Number of minutes to expire the configuration in [env: GIT_AUTHORS_TIMEOUT=]
                                       [default: 60]

ARGS:
    <author-initial>...    Initials of the authors to put in the commit
"#,
            env!("CARGO_PKG_VERSION"),
            default_config_file
        ),
        "",
        true,
    )
}

#[test]
fn help_returned_by_short_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-authors", vec!["-h"]);
    let default_config_file = config_file_path();
    assert_output(
        &output,
        &format!(
            r#"git-authors {}
Billie Thompson <billie+git-author@billiecodes.com>
Set author and Co-authored trailer.

USAGE:
    git-authors [OPTIONS] <author-initial>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <author-file-path>    Initials of the authors to put in the commit [env: GIT_AUTHORS_AUTHOR_FILE_PATH=]
                                       [default: {}]
    -t, --timeout <timeout>            Number of minutes to expire the configuration in [env: GIT_AUTHORS_TIMEOUT=]
                                       [default: 60]

ARGS:
    <author-initial>...    Initials of the authors to put in the commit
"#,
            env!("CARGO_PKG_VERSION"),
            default_config_file
        ),
        "",
        true,
    )
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
    let expected = r#"error: Found argument '--banana' which wasn't expected, or isn't valid in this context

USAGE:
    git-authors [OPTIONS] <author-initial>...

For more information try --help
"#;

    assert_output(&output, "", expected, false)
}
