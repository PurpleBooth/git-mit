use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// An author that might be developing
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Author<'a> {
    name: Cow<'a, str>,
    email: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    signingkey: Option<Cow<'a, str>>,
}

impl<'a> Author<'a> {
    /// Create a new author
    #[must_use]
    pub const fn new(
        name: Cow<'a, str>,
        email: Cow<'a, str>,
        signingkey: Option<Cow<'a, str>>,
    ) -> Self {
        Self {
            name,
            email,
            signingkey,
        }
    }

    /// The author name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The authors email
    #[must_use]
    pub fn email(&self) -> &str {
        &self.email
    }

    /// The authors gpg key
    #[must_use]
    pub fn signingkey(&self) -> Option<&str> {
        self.signingkey.as_deref()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use super::*;

    #[test]
    fn test_new_author_creation() {
        let author = Author::new("The Name".into(), "email@example.com".into(), None);

        assert_eq!(
            author.name(),
            "The Name",
            "Expected the author's name to be 'The Name'"
        );
        assert_eq!(
            author.email(),
            "email@example.com",
            "Expected the author's email to be 'email@example.com'"
        );
        assert_eq!(
            author.signingkey(),
            None,
            "Expected the author's signing key to be None when not set"
        );
    }

    #[test]
    fn test_author_with_signing_key() {
        let author = Author::new(
            "The Name".into(),
            "email@example.com".into(),
            Some("0A46826A".into()),
        );

        assert_eq!(
            author.signingkey(),
            Some("0A46826A"),
            "Expected the author's signing key to be '0A46826A'"
        );
    }
}
