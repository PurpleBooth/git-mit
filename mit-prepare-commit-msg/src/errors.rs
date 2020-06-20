use mit_commit_message_lints::author::VcsError;
use mit_commit_message_lints::external;
use mit_commit_message_lints::lints::CommitMessageError;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MitPrepareCommitMessageError {
    #[error("{0}")]
    MitCommitMessageLintsError(#[from] CommitMessageError),
    #[error("Failed to read author config from `{0}`:\n{1}")]
    Io(String, String),
    #[error("Expected commit file path")]
    MissingCommitFilePath,
    #[error("{0}")]
    AuthorWrite(#[from] VcsError),
    #[error("{0}")]
    ReadFromVcs(#[from] external::Error),
}

impl MitPrepareCommitMessageError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> MitPrepareCommitMessageError {
        MitPrepareCommitMessageError::Io(source, format!("{}", error))
    }
}
