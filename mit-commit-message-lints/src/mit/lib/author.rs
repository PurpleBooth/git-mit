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
    pub fn new(name: &str, email: &str, signingkey: Option<&str>) -> Self {
        Self {
            name: name.into(),
            email: email.into(),
            signingkey: signingkey.map(std::convert::Into::into),
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
