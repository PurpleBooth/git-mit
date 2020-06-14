use mit_hook_test_helper::assert_output;

#[test]
fn long_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "mit-commit-msg", vec!["--version"]);
    assert_output(
        &output,
        &format!("mit-commit-msg {}", env!("CARGO_PKG_VERSION")),
        "",
        true,
    )
}

#[test]
fn short_flag() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "mit-commit-msg", vec!["-V"]);
    assert_output(
        &output,
        &format!("mit-commit-msg {}", env!("CARGO_PKG_VERSION")),
        "",
        true,
    )
}
