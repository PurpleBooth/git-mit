//! Rotation strategy for the primary author across commits
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::mit::lib::errors::DeserializeRotationOptionError;

/// How to rotate the primary author when pairing or mobbing
///
/// When new strategies are added (e.g. random) they will appear as
/// additional variants here.
#[derive(clap::ValueEnum, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub enum RotationOption {
    /// Rotate through authors in order, one per commit
    RoundRobin,
    /// Shuffle authors randomly on each commit
    Random,
}

const ROUND_ROBIN_DISPLAY: &str = "round-robin";
const RANDOM_DISPLAY: &str = "random";

impl FromStr for RotationOption {
    type Err = DeserializeRotationOptionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            ROUND_ROBIN_DISPLAY => Ok(Self::RoundRobin),
            RANDOM_DISPLAY => Ok(Self::Random),
            _ => Err(DeserializeRotationOptionError { src: s.into() }),
        }
    }
}

impl Display for RotationOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RoundRobin => write!(f, "{ROUND_ROBIN_DISPLAY}"),
            Self::Random => write!(f, "{RANDOM_DISPLAY}"),
        }
    }
}
