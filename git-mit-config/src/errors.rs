use mit_commit_message_lints::errors::MitCommitMessageLintsError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GitMitConfigError {
    ConfigIoGit2Error(String),
    UnrecognisedLintCommand,
    LintNameNotGiven,
    MitCommitMessageLintsError(MitCommitMessageLintsError),
    Io(String, String),
}

impl Display for GitMitConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitMitConfigError::UnrecognisedLintCommand => write!(
                f,
                "Unrecognised Lint command, only you may only enable or disable, or list \
             available lints"
            ),
            GitMitConfigError::LintNameNotGiven => write!(f, "Please specify a lint"),
            GitMitConfigError::MitCommitMessageLintsError(error) => write!(f, "{}", error),
            GitMitConfigError::Io(file_source, error) => write!(
                f,
                "Failed to read git config from `{}`:\n{}",
                file_source, error
            ),
            GitMitConfigError::ConfigIoGit2Error(error) => {
                write!(f, "Failed to load a git config: {}", error)
            }
        }
    }
}

impl From<git2::Error> for GitMitConfigError {
    fn from(error: git2::Error) -> Self {
        GitMitConfigError::ConfigIoGit2Error(format!("{}", error))
    }
}

impl From<MitCommitMessageLintsError> for GitMitConfigError {
    fn from(from: MitCommitMessageLintsError) -> Self {
        GitMitConfigError::MitCommitMessageLintsError(from)
    }
}

impl Error for GitMitConfigError {}

impl GitMitConfigError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> GitMitConfigError {
        GitMitConfigError::Io(source, format!("{}", error))
    }
}
