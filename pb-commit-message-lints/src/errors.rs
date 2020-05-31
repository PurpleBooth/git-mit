use serde::export::Formatter;
use std::{
    error::Error,
    fmt::{Display, Result as FmtResult},
    num::ParseIntError,
    str::ParseBoolError,
};

#[derive(Debug, Eq, PartialEq)]
pub enum PbCommitMessageLintsError {
    ConfigIoGit2Error(String),
    ConfigIoInMemoryError,
    ParseBoolError(std::str::ParseBoolError),
    ParseIntError(std::num::ParseIntError),
    SystemTimeError(String),
    FromIntegerError(std::num::TryFromIntError),
    NoAuthorsToSetError,
    YamlParseError(String),
}

impl Display for PbCommitMessageLintsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PbCommitMessageLintsError::ConfigIoGit2Error(error) => {
                write!(f, "Couldn't interact with git config: {}", error)
            },
            PbCommitMessageLintsError::ConfigIoInMemoryError => {
                write!(f, "Couldn't interact with in memory config")
            },
            PbCommitMessageLintsError::ParseBoolError(error) => {
                write!(f, "Couldn't convert value to bool: {} ({:?})", error, error)
            },
            PbCommitMessageLintsError::ParseIntError(error) => {
                write!(f, "Couldn't convert value to int: {} ({:?})", error, error)
            },
            PbCommitMessageLintsError::SystemTimeError(error) => {
                write!(f, "Invalid time: {}", error)
            },
            PbCommitMessageLintsError::FromIntegerError(error) => write!(
                f,
                "Failed to convert between integer types: {} ({:?})",
                error, error
            ),
            PbCommitMessageLintsError::NoAuthorsToSetError => write!(
                f,
                "In order to set authors, you must provide at least one author to set"
            ),
            PbCommitMessageLintsError::YamlParseError(error) => {
                write!(f, "Couldn't parse the Author YAML: {}", error)
            },
        }
    }
}

impl From<git2::Error> for PbCommitMessageLintsError {
    fn from(error: git2::Error) -> Self {
        PbCommitMessageLintsError::ConfigIoGit2Error(format!("{} ({:?})", error, error))
    }
}

impl From<std::str::ParseBoolError> for PbCommitMessageLintsError {
    fn from(error: ParseBoolError) -> Self {
        PbCommitMessageLintsError::ParseBoolError(error)
    }
}

impl From<std::num::ParseIntError> for PbCommitMessageLintsError {
    fn from(error: ParseIntError) -> Self {
        PbCommitMessageLintsError::ParseIntError(error)
    }
}

impl From<std::time::SystemTimeError> for PbCommitMessageLintsError {
    fn from(error: std::time::SystemTimeError) -> Self {
        PbCommitMessageLintsError::SystemTimeError(format!("{} ({:?})", error, error))
    }
}

impl From<std::num::TryFromIntError> for PbCommitMessageLintsError {
    fn from(error: std::num::TryFromIntError) -> Self {
        PbCommitMessageLintsError::FromIntegerError(error)
    }
}

impl From<serde_yaml::Error> for PbCommitMessageLintsError {
    fn from(error: serde_yaml::Error) -> Self {
        PbCommitMessageLintsError::YamlParseError(format!("{} ({:?})", error, error))
    }
}

impl Error for PbCommitMessageLintsError {}
