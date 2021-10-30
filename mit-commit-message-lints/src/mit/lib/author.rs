use serde::{Deserialize, Serialize};

/// An author that might be developing
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Author {
    name: String,
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    signingkey: Option<String>,
}

impl Author {
    /// Create a new author
    #[must_use]
    pub fn new(name: &str, email: &str, signingkey: Option<&str>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            signingkey: signingkey.map(std::convert::Into::into),
        }
    }

    /// The authors name
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// The authors email
    #[must_use]
    pub fn email(&self) -> String {
        self.email.clone()
    }

    /// The authors gpg key
    #[must_use]
    pub fn signingkey(&self) -> Option<String> {
        self.signingkey.clone()
    }
}
