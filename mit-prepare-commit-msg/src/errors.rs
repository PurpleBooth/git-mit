use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum MitPrepareCommitMessageError {
    #[error("Expected commit file path")]
    #[diagnostic(code(mit_prepare_commit_msg::errors::missing_commit_file_path))]
    MissingCommitFilePath,

    #[error("The relates-to exec command failed with exit code {exit_code}")]
    #[diagnostic(code(mit_prepare_commit_msg::errors::relates_to_exec_failed))]
    RelatesToExecFailed { exit_code: i32 },
}
