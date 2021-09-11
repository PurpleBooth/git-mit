use std::{convert::Infallible, string};

use mit_commit_message_lints::{external, mit};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitMitError {
    #[error("failed convert to unicode: {0}")]
    Utf8(#[from] string::FromUtf8Error),
    #[error("failed to interact with git repository: {0}")]
    Git2Io(#[from] external::Error),
    #[error("no mit initials provided")]
    NoAuthorInitialsProvided,
    #[error("no timeout set")]
    NoTimeoutSet,
    #[error("timeout needs to be the number of minutes: {0}")]
    TimeoutNotNumber(#[from] std::num::ParseIntError),
    #[error("expected a mit file path, didn't find one")]
    AuthorFileNotSet,
    #[error("failed to read config from `{0}`: {1}")]
    Io(String, String),
    #[error("failed to generate config with `{0}`: {1}")]
    Exec(String, String),
    #[cfg(not(target_os = "windows"))]
    #[error("failed to calculate config directory {0}")]
    Xdg(#[from] xdg::BaseDirectoriesError),
    #[error("failed to parse mit author config {0}")]
    AuthorConfigParse(#[from] mit::AuthorConfigParseError),
    #[error("failed to set mit in vcs {0}")]
    AuthorVcs(#[from] mit::VcsError),
    #[error("appdata environment variable missing {0}")]
    AppDataMissing(#[from] std::env::VarError),
    #[error("failed to parse shell given {0}")]
    BadShellCommand(#[from] shell_words::ParseError),
    #[error("{0}")]
    Infallible(#[from] Infallible),
}

impl GitMitError {
    pub(crate) fn new_io(source: String, error: &std::io::Error) -> GitMitError {
        GitMitError::Io(source, format!("{}", error))
    }

    pub(crate) fn new_pwd_io(error: &std::io::Error) -> GitMitError {
        GitMitError::Io("$PWD".into(), format!("{}", error))
    }

    pub(crate) fn new_exec(source: String, error: &std::io::Error) -> GitMitError {
        GitMitError::Exec(source, format!("{}", error))
    }
}
