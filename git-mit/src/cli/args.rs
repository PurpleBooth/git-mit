use crate::errors::GitMitError;
use clap::ArgMatches;
use std::env;
use std::path::PathBuf;

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
        env::current_dir().map_err(|error| GitMitError::new_io("$PWD".into(), &error))
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

    pub fn initials(&self) -> Option<Vec<&str>> {
        self.matches.values_of("initials").map(Iterator::collect)
    }

    pub fn author_file(&self) -> Option<&str> {
        self.matches.value_of("file")
    }
}