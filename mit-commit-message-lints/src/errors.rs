use serde::export::Formatter;
use std::{
    error::Error,
    fmt::{Display, Result as FmtResult},
    num::ParseIntError,
    str::ParseBoolError,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum MitCommitMessageLintsError {
    ConfigIoGit2Error(String),
    ParseBoolError(std::str::ParseBoolError),
    ParseIntError(std::num::ParseIntError),
    SystemTimeError(String),
    FromIntegerError(std::num::TryFromIntError),
    NoAuthorsToSetError,
    LintNotFoundError(String),
    YamlParseError(String),
    IoError(String),
}

impl Display for MitCommitMessageLintsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            MitCommitMessageLintsError::ConfigIoGit2Error(error) => {
                write!(f, "Couldn't interact with git config:\n{}", error)
            }
            MitCommitMessageLintsError::ParseBoolError(error) => {
                write!(f, "Couldn't convert value to bool:\n{}", error)
            }
            MitCommitMessageLintsError::ParseIntError(error) => {
                write!(f, "Couldn't convert value to int:\n{}", error)
            }
            MitCommitMessageLintsError::SystemTimeError(error) => {
                write!(f, "Invalid time:\n{}", error)
            }
            MitCommitMessageLintsError::FromIntegerError(error) => {
                write!(f, "Failed to convert between integer types:\n{}", error)
            }
            MitCommitMessageLintsError::NoAuthorsToSetError => write!(
                f,
                "In order to set mit, you must provide at least one author to set"
            ),
            MitCommitMessageLintsError::LintNotFoundError(error) => {
                write!(f, "Lint \"{}\" not found", error)
            }
            MitCommitMessageLintsError::YamlParseError(error) => {
                write!(f, "Couldn't parse the Author YAML:\n{}", error)
            }
            MitCommitMessageLintsError::IoError(error) => {
                write!(f, "Failed to read file:\n{}", error)
            }
        }
    }
}

impl From<git2::Error> for MitCommitMessageLintsError {
    fn from(error: git2::Error) -> Self {
        MitCommitMessageLintsError::ConfigIoGit2Error(format!("{}", error))
    }
}

impl From<std::str::ParseBoolError> for MitCommitMessageLintsError {
    fn from(error: ParseBoolError) -> Self {
        MitCommitMessageLintsError::ParseBoolError(error)
    }
}

impl From<std::num::ParseIntError> for MitCommitMessageLintsError {
    fn from(error: ParseIntError) -> Self {
        MitCommitMessageLintsError::ParseIntError(error)
    }
}

impl From<std::time::SystemTimeError> for MitCommitMessageLintsError {
    fn from(error: std::time::SystemTimeError) -> Self {
        MitCommitMessageLintsError::SystemTimeError(format!("{}", error))
    }
}

impl From<std::num::TryFromIntError> for MitCommitMessageLintsError {
    fn from(error: std::num::TryFromIntError) -> Self {
        MitCommitMessageLintsError::FromIntegerError(error)
    }
}

impl From<serde_yaml::Error> for MitCommitMessageLintsError {
    fn from(error: serde_yaml::Error) -> Self {
        MitCommitMessageLintsError::YamlParseError(format!("{}", error))
    }
}

impl From<std::io::Error> for MitCommitMessageLintsError {
    fn from(error: std::io::Error) -> Self {
        MitCommitMessageLintsError::IoError(format!("{}", error))
    }
}

impl Error for MitCommitMessageLintsError {}
