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
            .and_then(|timeout| timeout.parse().map_err(GitMitError::from))
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

#[cfg(test)]
mod tests {
    use std::env;

    use quickcheck::TestResult;

    use super::{super::app::app, Args};

    #[test]
    fn can_get_cwd() {
        assert_eq!(Args::cwd().unwrap(), env::current_dir().unwrap());
    }

    #[quickcheck]
    fn timeout_will_be_ok_with_valid_u64(timeout: u64) -> bool {
        Some(timeout)
            == Args::from(app().get_matches_from(vec![
                "git-mit",
                "--timeout",
                &format!("{}", timeout),
                "eg",
            ]))
            .timeout()
            .ok()
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn timeout_will_fail_without_valid_u64(timeout: String) -> TestResult {
        if timeout.parse::<u64>().is_ok() {
            return TestResult::discard();
        }

        if timeout.starts_with('-') {
            return TestResult::discard();
        }

        TestResult::from_bool(
            Args::from(app().get_matches_from(vec!["git-mit", "--timeout", &timeout, "eg"]))
                .timeout()
                .is_err(),
        )
    }
}
