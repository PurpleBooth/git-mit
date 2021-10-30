//! Command errors
use std::string::FromUtf8Error;

use miette::Diagnostic;
use thiserror::Error;

/// Command error
#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    /// No authors provided to set
    #[error("no authors provided to set")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::mit::cmd::errors::error::no_authors_to_set)
    )]
    NoAuthorsToSet,
    /// Failed to convert author command output to unicode
    #[error("failed to convert author command output to unicode")]
    #[diagnostic(
        url(docsrs),
        code(git_mit::errors::git_mit_error::exec_utf8),
        help("all characters must parse as utf8")
    )]
    ExecUtf8 {
        /// The command we ran that failed
        #[source_code]
        command: String,
        /// The error itself
        #[source]
        source: FromUtf8Error,
    },
    /// No mit initials provided
    #[error("no mit initials provided")]
    #[diagnostic(
        url(docsrs),
        code(git_mit::errors::git_mit_error::no_author_initials_provided)
    )]
    NoAuthorInitialsProvided,
    /// No timeout set
    #[error("no timeout set")]
    #[diagnostic(url(docsrs), code(git_mit::errors::git_mit_error::no_timeout_set))]
    NoTimeoutSet,
    /// Expected a mit file path, didn't find one
    #[error("expected a mit file path, didn't find one")]
    #[diagnostic(url(docsrs), code(git_mit::errors::git_mit_error::author_file_not_set))]
    AuthorFileNotSet,
}
