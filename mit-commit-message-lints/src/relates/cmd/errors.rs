use std::{num, time};

use miette::Diagnostic;
use thiserror::Error;

use crate::external;

/// Overall errors for this binary
#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    /// A failure from libgit2
    #[error("failed to interact with the git config: {0}")]
    #[diagnostic(transparent)]
    GitIo(external::Error),

    /// Out of range integral type conversion attempted
    #[error("failed converted epoch int between types: {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::relates::vcs::error::epoch_convert)
    )]
    EpochConvert(num::TryFromIntError),

    /// An error reading the system time
    #[error("failed to get system time: {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::relates::vcs::error::system_time)
    )]
    SystemTime(time::SystemTimeError),
}
