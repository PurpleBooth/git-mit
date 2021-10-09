use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum MitPrepareCommitMessageError {
    #[error("Expected commit file path")]
    #[diagnostic(code(mit_prepare_commit_msg::errors::missing_commit_file_path))]
    MissingCommitFilePath,
}
