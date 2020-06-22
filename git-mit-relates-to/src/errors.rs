use mit_commit_message_lints::{external, relates};
use std::{io, num, string};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitRelatesTo {
    #[error("relates to should be a number in minutes {0}")]
    RelatesToNotNumber(#[from] num::ParseIntError),
    #[error("failed convert to unicode: {0}")]
    Utf8(#[from] string::FromUtf8Error),
    #[error("failed to interact with the vcs: {0}")]
    RelatesVcs(#[from] relates::VcsError),
    #[error("{0}")]
    External(#[from] external::Error),
    #[error("not timeout set")]
    NoTimeoutSet,
    #[error("not relates to message set")]
    NoRelatesToMessageSet,
    #[error("could not get current directory {0}")]
    Io(#[from] io::Error),
}
