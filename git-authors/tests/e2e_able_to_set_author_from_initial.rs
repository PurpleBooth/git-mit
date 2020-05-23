use pb_hook_test_helper::assert_output;

use git2::Repository;
use std::io::Write;

#[test]
fn no_authors_fail() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-authors", vec![]);
    assert_output(
        &output,
        "",
        "error: The following required arguments were not provided:
    <author-initial>...

USAGE:
    git-authors <author-initial>... --config <author-file-path> --timeout <timeout>

For more information try --help
",
        false,
    )
}

#[test]
fn one_initial_sets_that_initial_as_author() {
    let mut author_config =
        tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    author_config
        .write_all(
            r#"
---
bt:
    name: Billie Thompson
    email: billie@example.com
"#
            .as_bytes(),
        )
        .expect("Failed to write to temporary author config");

    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "git-authors",
        vec![
            "-c",
            author_config
                .path()
                .to_str()
                .expect("Failed to convert path to string"),
            "bt",
        ],
    );

    let repository = Repository::open(working_dir).expect("Failed to open repository");
    let config = repository
        .config()
        .expect("Failed to open repository config")
        .snapshot()
        .unwrap();
    let actual_author_name = config
        .get_str("user.name")
        .expect("Failed to read username");
    let actual_author_email = config.get_str("user.email").expect("Failed to read email");

    assert_eq!(actual_author_name, "Billie Thompson");
    assert_eq!(actual_author_email, "billie@example.com");

    assert_output(&output, "", "", true);
}
