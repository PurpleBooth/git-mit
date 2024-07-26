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
        match s {
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
