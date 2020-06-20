use mit_commit_message_lints::{author, external, lints};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitMitConfigError {
    #[error("Lint not found: {0}")]
    CommitMessageReadError(#[from] lints::CommitMessageError),
    #[error("lint name not given")]
    LintNameNotGiven,
    #[error("failed to parse author yaml {0}")]
    AuthorYaml(#[from] author::YamlError),
    #[error("failed to read config from `{0}`: {1}")]
    Io(String, String),
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
}

impl GitMitConfigError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> GitMitConfigError {
        GitMitConfigError::Io(source, format!("{}", error))
    }
}
