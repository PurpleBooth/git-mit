use std::string;

use thiserror::Error;

use mit_commit_message_lints::{author, external};

#[derive(Error, Debug)]
pub enum GitMitError {
    #[error("failed convert to unicode: {0}")]
    Utf8(#[from] string::FromUtf8Error),
    #[error("failed to interact with git repository: {0}")]
    Git2Io(#[from] external::Error),
    #[error("no author initials provided")]
    NoAuthorInitialsProvided,
    #[error("no timeout set")]
    NoTimeoutSet,
    #[error("timeout needs to be the number of minutes: {0}")]
    TimeoutNotNumber(#[from] std::num::ParseIntError),
    #[error("expected a author file path, didn't find one")]
    AuthorFileNotSet,
    #[error("failed to read config from `{0}`: {1}")]
    Io(String, String),
    #[error("failed to generate config with `{0}`: {1}")]
    Exec(String, String),
    #[error("failed to calculate config directory {0}")]
    Xdg(#[from] xdg::BaseDirectoriesError),
    #[error("failed to parse author yaml {0}")]
    AuthorYaml(#[from] author::YamlError),
    #[error("failed to set author in vcs {0}")]
    AuthorVcs(#[from] author::VcsError),
}

impl GitMitError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> GitMitError {
        GitMitError::Io(source, format!("{}", error))
    }

    pub(crate) fn new_exec(source: String, error: &std::io::Error) -> GitMitError {
        GitMitError::Exec(source, format!("{}", error))
    }
}
