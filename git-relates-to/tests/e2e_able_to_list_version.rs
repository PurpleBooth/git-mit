use mit_hook_test_helper::assert_output;

#[test]
fn version_returned_by_long_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "git-relates-to", vec!["--version"]);
    assert_output(
        &output,
        &format!("git-relates-to {}", env!("CARGO_PKG_VERSION")),
        "",
        true,
    )
}

#[test]
fn version_returned_by_short_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "git-relates-to", vec!["-V"]);
    assert_output(
        &output,
        &format!("git-relates-to {}", env!("CARGO_PKG_VERSION")),
        "",
        true,
    )
}
