use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum GitRelatesTo {
    #[error("not relates to message set")]
    #[diagnostic(code(git_mit_relates_to::errors::git_relates_to::no_relates_to_message_set))]
    NoRelatesToMessageSet,
}
