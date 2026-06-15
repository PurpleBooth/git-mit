//! Rotation strategy for the primary author across commits
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::mit::lib::errors::DeserializeRotationOptionError;

/// How to rotate the primary author when pairing or mobbing
#[derive(clap::ValueEnum, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub enum RotationOption {
    /// Rotation is disabled
    Off,
    /// Rotate through authors in order, one per commit
    RoundRobin,
    /// Shuffle authors randomly on each commit
    Random,
}

const OFF_DISPLAY: &str = "off";
const ROUND_ROBIN_DISPLAY: &str = "round-robin";
const RANDOM_DISPLAY: &str = "random";

impl FromStr for RotationOption {
    type Err = DeserializeRotationOptionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            OFF_DISPLAY => Ok(Self::Off),
            ROUND_ROBIN_DISPLAY => Ok(Self::RoundRobin),
            RANDOM_DISPLAY => Ok(Self::Random),
            _ => Err(DeserializeRotationOptionError { src: s.into() }),
        }
    }
}

impl Display for RotationOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => write!(f, "{OFF_DISPLAY}"),
            Self::RoundRobin => write!(f, "{ROUND_ROBIN_DISPLAY}"),
            Self::Random => write!(f, "{RANDOM_DISPLAY}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::RotationOption;

    #[test]
    fn from_str_accepts_lowercase() {
        assert_eq!(
            RotationOption::from_str("round-robin").unwrap(),
            RotationOption::RoundRobin,
            "Expected 'round-robin' to parse as RoundRobin"
        );
    }

    #[test]
    fn from_str_rejects_unknown() {
        assert!(
            RotationOption::from_str("unknown").is_err(),
            "Expected parsing an unknown rotation option to return an error"
        );
    }

    #[test]
    fn from_str_is_case_insensitive_like_value_enum() {
        assert_eq!(
            RotationOption::from_str("Round-Robin").unwrap(),
            RotationOption::RoundRobin,
            "Expected 'Round-Robin' to parse as RoundRobin (case insensitive)"
        );
        assert_eq!(
            RotationOption::from_str("ROUND-ROBIN").unwrap(),
            RotationOption::RoundRobin,
            "Expected 'ROUND-ROBIN' to parse as RoundRobin (case insensitive)"
        );
    }

    #[test]
    fn from_str_accepts_random() {
        assert_eq!(
            RotationOption::from_str("random").unwrap(),
            RotationOption::Random,
            "Expected 'random' to parse as Random"
        );
    }

    #[test]
    fn from_str_accepts_random_case_insensitive() {
        assert_eq!(
            RotationOption::from_str("Random").unwrap(),
            RotationOption::Random,
            "Expected 'Random' to parse as Random (case insensitive)"
        );
        assert_eq!(
            RotationOption::from_str("RANDOM").unwrap(),
            RotationOption::Random,
            "Expected 'RANDOM' to parse as Random (case insensitive)"
        );
    }

    #[test]
    fn display_round_trips_through_from_str() {
        for original in [
            RotationOption::Off,
            RotationOption::RoundRobin,
            RotationOption::Random,
        ] {
            let displayed = original.to_string();
            let parsed = RotationOption::from_str(&displayed);
            assert_eq!(
                parsed.unwrap(),
                original,
                "Expected display output to round-trip through from_str"
            );
        }
    }

    #[test]
    fn display_random_round_trips() {
        let original = RotationOption::Random;
        let displayed = original.to_string();
        let parsed = RotationOption::from_str(&displayed);
        assert_eq!(
            parsed.unwrap(),
            original,
            "Expected Random display output to round-trip through from_str"
        );
    }
}
