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
}
