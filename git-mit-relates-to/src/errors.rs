use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum GitRelatesTo {
    #[error("not timeout set")]
    #[diagnostic()]
    NoTimeoutSet,
    #[error("not relates to message set")]
    #[diagnostic()]
    NoRelatesToMessageSet,
}
