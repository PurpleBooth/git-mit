use std::{env, path::PathBuf, str::FromStr};

use clap::ArgMatches;
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{console::completion::Shell, mit::AuthorArgs};

use crate::errors::GitMitError;

pub struct Args {
    matches: ArgMatches,
}

impl From<ArgMatches> for Args {
    fn from(matches: ArgMatches) -> Self {
        Self { matches }
    }
}

impl Args {
    pub(crate) fn cwd() -> Result<PathBuf> {
        env::current_dir().into_diagnostic()
    }

    pub(crate) fn timeout(&self) -> Result<u64> {
        match self.matches.value_of("timeout") {
            None => Err(GitMitError::NoTimeoutSet.into()),
            Some(value) => Ok(value),
        }
        .and_then(|timeout| timeout.parse().into_diagnostic())
    }

    pub fn initials(&self) -> Result<Vec<&str>> {
        match self.matches.values_of("initials") {
            None => Err(GitMitError::NoAuthorInitialsProvided.into()),
            Some(value) => Ok(value.collect()),
        }
    }

    pub fn completion(&self) -> Option<Shell> {
        self.matches
            .value_of("completion")
            .and_then(|shell_name| Shell::from_str(shell_name).ok())
    }
}

impl AuthorArgs for Args {
    fn author_command(&self) -> Option<&str> {
        self.matches.value_of("command")
    }

    fn author_file(&self) -> Option<&str> {
        self.matches.value_of("file")
    }
}
