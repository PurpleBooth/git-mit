use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// User input data for the relates to trailer
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RelateTo<'a> {
    relates: Cow<'a, str>,
}

impl<'a> RelateTo<'a> {
    /// Create a new relates to
    #[must_use]
    pub const fn new(relates: Cow<'a, str>) -> Self {
        Self { relates }
    }

    /// What this relates to
    #[must_use]
    pub fn to(&self) -> &str {
        &self.relates
    }
}

impl<'a> From<&'a str> for RelateTo<'a> {
    fn from(input: &'a str) -> Self {
        RelateTo {
            relates: Cow::Borrowed(input),
        }
    }
}
impl From<String> for RelateTo<'_> {
    fn from(input: String) -> Self {
        RelateTo {
            relates: Cow::Owned(input),
        }
    }
}
