use std::io;

use thiserror::Error;

use mit_commit_message_lints::mit::VcsError;
use mit_commit_message_lints::{external, lints, mit};

#[derive(Error, Debug)]
pub enum GitMitConfigError {
    #[error("lint name not given")]
    LintNameNotGiven,
    #[error("author file not set")]
    AuthorFileNotSet,
    #[error("failed to parse mit author config {0}")]
    AuthorConfigParse(#[from] mit::AuthorConfigParseError),
    #[error("failed to open git repository {0}")]
    Git2(#[from] git2::Error),
    #[error("{0}")]
    LintsError(#[from] lints::Error),
    #[error(
        "Unrecognised Lint command, only you may only enable or disable, or list available lints"
    )]
    UnrecognisedLintCommand,
    #[error("{0}")]
    SetStatus(#[from] lints::SetStatusError),
    #[error("{0}")]
    External(#[from] external::Error),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Vcs(#[from] VcsError),
    #[error("{0}")]
    Xdg(#[from] xdg::BaseDirectoriesError),
    #[error("{0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}
