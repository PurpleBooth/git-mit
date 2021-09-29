use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("no authors provided to set")]
    #[diagnostic()]
    NoAuthorsToSet,
}
