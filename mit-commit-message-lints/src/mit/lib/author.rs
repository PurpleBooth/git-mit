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

    /// The authors name
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
