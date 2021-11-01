#![allow(clippy::wildcard_imports)]

use super::author::*;

#[test]
fn has_an_author() {
    let author = Author::new("The Name".into(), "email@example.com".into(), None);

    assert_eq!(author.name(), "The Name");
    assert_eq!(author.email(), "email@example.com");
    assert_eq!(author.signingkey(), None);
}

#[test]
fn has_an_signing_key() {
    let author = Author::new(
        "The Name".into(),
        "email@example.com".into(),
        Some("0A46826A".into()),
    );

    assert_eq!(author.signingkey(), Some("0A46826A"));
}
