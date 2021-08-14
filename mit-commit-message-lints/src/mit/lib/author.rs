use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Author {
    name: String,
    email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    signingkey: Option<String>,
}

impl Author {
    #[must_use]
    pub fn new(name: &str, email: &str, signingkey: Option<&str>) -> Author {
        Author {
            name: name.into(),
            email: email.into(),
            signingkey: signingkey.map(|key| key.into()),
        }
    }

    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub fn email(&self) -> String {
        self.email.clone()
    }

    #[must_use]
    pub fn signingkey(&self) -> Option<String> {
        self.signingkey.clone()
    }
}

#[cfg(test)]
mod tests_author {
    #![allow(clippy::wildcard_imports)]

    use super::*;

    #[test]
    fn has_an_author() {
        let author = Author::new("The Name", "email@example.com", None);

        assert_eq!(author.name(), "The Name");
        assert_eq!(author.email(), "email@example.com");
        assert_eq!(author.signingkey(), None);
    }

    #[test]
    fn has_an_signing_key() {
        let author = Author::new("The Name", "email@example.com", Some("0A46826A"));

        assert_eq!(author.signingkey(), Some("0A46826A".into()));
    }
}
