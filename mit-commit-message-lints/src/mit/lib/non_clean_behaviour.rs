//! Differing behaviours on rebase
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::mit::lib::errors::DeserializeRebaseBehaviourError;

/// Differing behaviours on non-clean repository
#[derive(clap::ValueEnum, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub enum BehaviourOption {
    /// Change the commit message to include the current author
    AddTo,
    /// Do not change the commit message
    NoChange,
}

const ADD_TO_DISPLAY: &str = "add-to";
const NO_CHANGE_DISPLAY: &str = "no-change";

impl FromStr for BehaviourOption {
    type Err = DeserializeRebaseBehaviourError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            ADD_TO_DISPLAY => Ok(Self::AddTo),
            NO_CHANGE_DISPLAY => Ok(Self::NoChange),
            _ => Err(DeserializeRebaseBehaviourError { src: s.into() }),
        }
    }
}

impl Display for BehaviourOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddTo => write!(f, "{ADD_TO_DISPLAY}"),
            Self::NoChange => write!(f, "{NO_CHANGE_DISPLAY}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::BehaviourOption;

    #[test]
    fn from_str_accepts_lowercase() {
        assert_eq!(
            BehaviourOption::from_str("add-to").unwrap(),
            BehaviourOption::AddTo
        );
        assert_eq!(
            BehaviourOption::from_str("no-change").unwrap(),
            BehaviourOption::NoChange
        );
    }

    #[test]
    fn from_str_rejects_unknown() {
        assert!(BehaviourOption::from_str("unknown").is_err());
    }

    #[test]
    fn from_str_is_case_insensitive_like_value_enum() {
        // clap::ValueEnum accepts any casing; FromStr should too
        assert_eq!(
            BehaviourOption::from_str("Add-To").unwrap(),
            BehaviourOption::AddTo
        );
        assert_eq!(
            BehaviourOption::from_str("ADD-TO").unwrap(),
            BehaviourOption::AddTo
        );
        assert_eq!(
            BehaviourOption::from_str("No-Change").unwrap(),
            BehaviourOption::NoChange
        );
        assert_eq!(
            BehaviourOption::from_str("NO-CHANGE").unwrap(),
            BehaviourOption::NoChange
        );
    }

    #[test]
    fn display_round_trips_through_from_str() {
        for original in [BehaviourOption::AddTo, BehaviourOption::NoChange] {
            let displayed = original.to_string();
            let parsed = BehaviourOption::from_str(&displayed);
            assert_eq!(parsed.unwrap(), original);
        }
    }
}
