use std::{env, path::PathBuf};

use clap::ArgMatches;

use crate::errors::{GitMitError, GitMitError::NoAuthorInitialsProvided};

pub struct Args {
    matches: ArgMatches,
}

impl From<ArgMatches> for Args {
    fn from(matches: ArgMatches) -> Self {
        Args { matches }
    }
}

impl Args {
    pub(crate) fn cwd() -> Result<PathBuf, GitMitError> {
        env::current_dir().map_err(|error| GitMitError::new_pwd_io(&error))
    }

    pub(crate) fn timeout(&self) -> Result<u64, GitMitError> {
        self.matches
            .value_of("timeout")
            .ok_or(GitMitError::NoTimeoutSet)
            .and_then(|x| x.parse().map_err(GitMitError::from))
    }

    pub fn command(&self) -> Option<&str> {
        self.matches.value_of("command")
    }

    pub fn initials(&self) -> Result<Vec<&str>, crate::GitMitError> {
        self.matches
            .values_of("initials")
            .map(Iterator::collect)
            .ok_or(NoAuthorInitialsProvided)
    }

    pub fn author_file(&self) -> Option<&str> {
        self.matches.value_of("file")
    }
}
