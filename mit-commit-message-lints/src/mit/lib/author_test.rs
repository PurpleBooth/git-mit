#![allow(clippy::wildcard_imports)]

use super::author::*;

#[test]
fn test_new_author_creation() {
    let author = Author::new("The Name".into(), "email@example.com".into(), None);

    assert_eq!(author.name(), "The Name");
    assert_eq!(author.email(), "email@example.com");
    assert_eq!(author.signingkey(), None);
}

#[test]
fn test_author_with_signing_key() {
    let author = Author::new(
        "The Name".into(),
        "email@example.com".into(),
        Some("0A46826A".into()),
    );

    assert_eq!(author.signingkey(), Some("0A46826A"));
}
