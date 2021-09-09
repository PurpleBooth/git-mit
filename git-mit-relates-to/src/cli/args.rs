use std::time::Duration;

use clap::ArgMatches;

use crate::errors::GitRelatesTo;

pub struct Args {
    matches: ArgMatches,
}

impl From<ArgMatches> for Args {
    fn from(matches: ArgMatches) -> Self {
        Args { matches }
    }
}

impl Args {
    pub(crate) fn issue_number(&self) -> Result<&str, GitRelatesTo> {
        self.matches
            .value_of("issue-number")
            .ok_or(GitRelatesTo::NoRelatesToMessageSet)
    }

    pub(crate) fn timeout(&self) -> Result<Duration, GitRelatesTo> {
        self.matches
            .value_of("timeout")
            .ok_or(GitRelatesTo::NoTimeoutSet)
            .and_then(|timeout| timeout.parse().map_err(GitRelatesTo::from))
            .map(|timeout: u64| timeout * 60)
            .map(Duration::from_secs)
    }
}
