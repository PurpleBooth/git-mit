use mit_commit_message_lints::errors::MitCommitMessageLintsError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GitMitError {
    NoAuthorInitialsProvided,
    NoTimeoutSet,
    PbCommitMessageLints(MitCommitMessageLintsError),
    Io(String, String),
    Exec(String, String),
    Xdg(String),
    TimeoutNotNumber(String),
    Utf8(String),
    AuthorFileNotSet,
}

impl GitMitError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> GitMitError {
        GitMitError::Io(source, format!("{}", error))
    }
    pub(crate) fn new_exec(source: String, error: &std::io::Error) -> GitMitError {
        GitMitError::Exec(source, format!("{}", error))
    }
}

impl Display for GitMitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitMitError::NoAuthorInitialsProvided => write!(f, "No author initials provided"),
            GitMitError::NoTimeoutSet => write!(f, "No timeout set"),
            GitMitError::TimeoutNotNumber(error) => write!(
                f,
                "The timeout needs to be the number of minutes:\n{}",
                error
            ),
            GitMitError::PbCommitMessageLints(error) => write!(f, "{}", error),
            GitMitError::Io(file_source, error) => {
                write!(f, "Failed to read from `{}`:\n{}", file_source, error)
            }
            GitMitError::Exec(exec, error) => write!(f, "Failed to run `{}`:\n{}", exec, error),
            GitMitError::Xdg(error) => write!(f, "Failed to find config directory: {}", error),
            GitMitError::AuthorFileNotSet => {
                write!(f, "Expected a author file path, didn't find one")
            }
            GitMitError::Utf8(error) => write!(
                f,
                "Failed to convert the output from the author file generation command to a UTF-8 \
             String:\n{}",
                error
            ),
        }
    }
}

impl From<std::string::FromUtf8Error> for GitMitError {
    fn from(from: std::string::FromUtf8Error) -> Self {
        GitMitError::Utf8(format!("{}", from))
    }
}

impl From<MitCommitMessageLintsError> for GitMitError {
    fn from(from: MitCommitMessageLintsError) -> Self {
        GitMitError::PbCommitMessageLints(from)
    }
}

impl From<std::num::ParseIntError> for GitMitError {
    fn from(from: std::num::ParseIntError) -> Self {
        GitMitError::TimeoutNotNumber(format!("{}", from))
    }
}

impl From<xdg::BaseDirectoriesError> for GitMitError {
    fn from(from: xdg::BaseDirectoriesError) -> Self {
        GitMitError::Xdg(format!("{}", from))
    }
}

impl Error for GitMitError {}
