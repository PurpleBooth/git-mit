use std::{num, time};

use thiserror::Error;

use crate::external;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to interact with git repository: {0}")]
    GitIo(#[from] external::Error),
    #[error("no authors provided to set")]
    NoAuthorsToSet,
    #[error("unable to read the current time {0}")]
    UnableToDetermineNow(#[from] time::SystemTimeError),
    #[error("unable to parse time {0}")]
    TimeInUnusualFormat(#[from] num::TryFromIntError),
}
