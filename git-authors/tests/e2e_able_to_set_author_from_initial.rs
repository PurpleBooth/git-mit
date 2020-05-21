use pb_hook_test_helper::assert_output;

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
