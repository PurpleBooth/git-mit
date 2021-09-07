use std::error;

use mit_commit::CommitMessageError;
use mit_commit_message_lints::{external, lints::Error as LintsError};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MitCommitMsgError {
    #[error("expected file path name")]
    CommitPathMissing,
    #[error("failed to read git config from `{0}`: {1}")]
    Io(String, String),
    #[error("{0}")]
    MitCommitMessageLint(#[from] LintsError),
    #[error("{0}")]
    MitCommitMessage(#[from] CommitMessageError),
    #[error("{0}")]
    External(#[from] external::Error),
    #[error("{0}")]
    Clipboard(#[from] Box<dyn error::Error + Sync + Send>),
}

impl MitCommitMsgError {
    pub(crate) fn new_io(location: String, error: &std::io::Error) -> MitCommitMsgError {
        MitCommitMsgError::Io(location, format!("{}", error))
    }
}
