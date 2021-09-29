use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub(crate) enum MitPrepareCommitMessageError {
    #[error("Expected commit file path")]
    #[diagnostic()]
    MissingCommitFilePath,
}
