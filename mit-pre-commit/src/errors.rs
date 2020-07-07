use thiserror::Error;

use mit_commit_message_lints::{external, mit};

#[derive(Error, Debug)]
pub(crate) enum MitPreCommitError {
    #[error("{0}")]
    MitCommitMessageLintsError(#[from] external::Error),
    #[error("{0}")]
    MitCommitMessageAuthorError(#[from] mit::VcsError),
    #[error("Failed to read config from `{0}`:\n{1}")]
    Io(String, String),
}

impl MitPreCommitError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> MitPreCommitError {
        MitPreCommitError::Io(source, format!("{}", error))
    }
}
