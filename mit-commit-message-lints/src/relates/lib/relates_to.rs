use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RelateTo {
    relates: String,
}

impl RelateTo {
    #[must_use]
    pub fn new(relates: &str) -> Self {
        Self {
            relates: relates.into(),
        }
    }

    #[must_use]
    pub fn to(&self) -> String {
        self.relates.clone()
    }
}
