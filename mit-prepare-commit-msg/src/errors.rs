use thiserror::Error;

use mit_commit::CommitMessageError;
use mit_commit_message_lints::{author, external, relates};

#[derive(Error, Debug)]
pub(crate) enum MitPrepareCommitMessageError {
    #[error("{0}")]
    MitCommitMessageLintsError(#[from] CommitMessageError),
    #[error("Failed to read author config from `{0}`:\n{1}")]
    Io(String, String),
    #[error("Expected commit file path")]
    MissingCommitFilePath,
    #[error("{0}")]
    AuthorWrite(#[from] author::VcsError),
    #[error("{0}")]
    RelatesToWrite(#[from] relates::VcsError),
    #[error("{0}")]
    ReadFromVcs(#[from] external::Error),
}

impl MitPrepareCommitMessageError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> MitPrepareCommitMessageError {
        MitPrepareCommitMessageError::Io(source, format!("{}", error))
    }
}
