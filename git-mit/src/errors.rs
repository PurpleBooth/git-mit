use std::string;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum GitMitError {
    #[error("failed to convert author command output to unicode")]
    #[diagnostic(
        code(git_mit::config::author::load),
        help("all characters must parse as utf8")
    )]
    ExecUtf8 {
        #[source_code]
        command: String,
        #[source]
        source: string::FromUtf8Error,
    },
    #[error("no mit initials provided")]
    NoAuthorInitialsProvided,
    #[error("no timeout set")]
    NoTimeoutSet,
    #[error("expected a mit file path, didn't find one")]
    AuthorFileNotSet,
}
