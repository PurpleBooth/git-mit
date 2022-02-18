use super::app::cli;

#[test]
fn package_name() {
    assert_eq!(cli().get_name(), env!("CARGO_PKG_NAME"));
}
