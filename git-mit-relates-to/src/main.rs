use std::{convert::TryFrom, env, time::Duration};

use clap::ArgMatches;

use mit_commit_message_lints::{
    external::Git2,
    relates::{entities::RelateTo, vcs::set_relates_to},
};

use crate::errors::GitRelatesTo;

mod cli;
mod errors;

fn main() -> Result<(), errors::GitRelatesTo> {
    let matches = cli::app().get_matches();

    let relates_to = matches
        .value_of("issue-number")
        .ok_or_else(|| GitRelatesTo::NoRelatesToMessageSet)?;

    let current_dir = env::current_dir()?;
    let mut vcs = Git2::try_from(current_dir)?;
    set_relates_to(
        &mut vcs,
        &RelateTo::new(relates_to),
        Duration::from_secs(get_timeout(&matches)? * 60),
    )?;

    Ok(())
}

fn get_timeout(matches: &ArgMatches) -> Result<u64, GitRelatesTo> {
    matches
        .value_of("timeout")
        .ok_or_else(|| GitRelatesTo::NoTimeoutSet)
        .and_then(|x| x.parse().map_err(GitRelatesTo::from))
}
