use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum MitPrepareCommitMessageError {
    #[error("The relates-to exec command failed with exit code {exit_code}")]
    #[diagnostic(code(mit_prepare_commit_msg::errors::relates_to_exec_failed))]
    RelatesToExecFailed { exit_code: i32 },

    #[error("invalid relates-to template")]
    #[diagnostic(
        code(mit_prepare_commit_msg::errors::invalid_relates_to_template),
        help(
            "the only supported placeholder is `{{ value }}`, everything else in the template is literal text"
        )
    )]
    InvalidRelatesToTemplate {
        #[source_code]
        src: String,
        #[label("unknown or unterminated placeholder")]
        span: SourceSpan,
    },
}
