use serde::{Deserialize, Serialize};

/// User input data for the relates to trailer
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RelateTo {
    relates: String,
}

impl RelateTo {
    /// Create a new relates to
    #[must_use]
    pub fn new(relates: &str) -> Self {
        Self {
            relates: relates.into(),
        }
    }

    /// What this relates to
    #[must_use]
    pub fn to(&self) -> String {
        self.relates.clone()
    }
}
