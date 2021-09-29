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
use miette::{IntoDiagnostic, Result};

impl Args {
    pub(crate) fn issue_number(&self) -> Result<&str> {
        self.matches
            .value_of("issue-number")
            .ok_or(GitRelatesTo::NoRelatesToMessageSet)
            .into_diagnostic()
    }

    pub(crate) fn timeout(&self) -> Result<Duration> {
        self.matches
            .value_of("timeout")
            .ok_or(GitRelatesTo::NoTimeoutSet)
            .into_diagnostic()
            .and_then(|timeout| timeout.parse().into_diagnostic())
            .map(|timeout: u64| timeout * 60)
            .map(Duration::from_secs)
    }
}
