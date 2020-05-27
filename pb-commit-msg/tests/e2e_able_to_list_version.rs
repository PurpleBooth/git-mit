use pb_hook_test_helper::assert_output;

#[test]
fn long_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-commit-msg", vec!["--version"]);
    assert_output(
        &output,
        &format!("pb-commit-msg {}\n", env!("CARGO_PKG_VERSION")),
        "",
        true,
    )
}

#[test]
fn short_flag() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "pb-commit-msg", vec!["-V"]);
    assert_output(
        &output,
        &format!("pb-commit-msg {}\n", env!("CARGO_PKG_VERSION")),
        "",
        true,
    )
}
