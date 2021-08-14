use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct RelateTo {
    relates: String,
}

impl RelateTo {
    #[must_use]
    pub fn new(relates: &str) -> RelateTo {
        RelateTo {
            relates: relates.into(),
        }
    }

    #[must_use]
    pub fn to(&self) -> String {
        self.relates.clone()
    }
}

#[cfg(test)]
mod tests_relate_to {
    #![allow(clippy::wildcard_imports)]

    use super::*;

    #[test]
    fn has_a_relate_to_string() {
        let relate = RelateTo::new("[#12343567]");

        assert_eq!(relate.to(), "[#12343567]");
    }
}
