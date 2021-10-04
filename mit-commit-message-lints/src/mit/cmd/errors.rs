use std::string::FromUtf8Error;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("no authors provided to set")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::mit::cmd::errors::error::no_authors_to_set)
    )]
    NoAuthorsToSet,
    #[error("failed to convert author command output to unicode")]
    #[diagnostic(
        url(docsrs),
        code(git_mit::errors::git_mit_error::exec_utf8),
        help("all characters must parse as utf8")
    )]
    ExecUtf8 {
        #[source_code]
        command: String,
        #[source]
        source: FromUtf8Error,
    },
    #[error("no mit initials provided")]
    #[diagnostic(
        url(docsrs),
        code(git_mit::errors::git_mit_error::no_author_initials_provided)
    )]
    NoAuthorInitialsProvided,
    #[error("no timeout set")]
    #[diagnostic(url(docsrs), code(git_mit::errors::git_mit_error::no_timeout_set))]
    NoTimeoutSet,
    #[error("expected a mit file path, didn't find one")]
    #[diagnostic(url(docsrs), code(git_mit::errors::git_mit_error::author_file_not_set))]
    AuthorFileNotSet,
}
