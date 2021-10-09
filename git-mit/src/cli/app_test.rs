use super::app::app;

#[test]
fn package_name() {
    assert_eq!(app().get_name(), env!("CARGO_PKG_NAME"));
}
