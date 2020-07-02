use std::io;

use thiserror::Error;

use mit_commit_message_lints::{author, external, lints};

#[derive(Error, Debug)]
pub enum GitMitConfigError {
    #[error("lint name not given")]
    LintNameNotGiven,
    #[error("failed to parse author yaml {0}")]
    AuthorYaml(#[from] author::YamlError),
    #[error("failed to open git repository {0}")]
    Git2(#[from] git2::Error),
    #[error("{0}")]
    LintsError(#[from] lints::LintsError),
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
}
