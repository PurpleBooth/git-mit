use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MitPrepareCommitMessageError {
    #[error("Expected commit file path")]
    MissingCommitFilePath,
}
