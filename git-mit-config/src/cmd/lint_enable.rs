use std::{convert::TryInto, env::current_dir, option::Option::None};

use clap::ArgMatches;
use mit_commit_message_lints::{external, lints::Lints};

use crate::{
    errors::{GitMitConfigError, GitMitConfigError::LintNameNotGiven},
    get_vcs,
};

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("enable").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<(), GitMitConfigError> {
    let subcommand = matches
        .subcommand_matches("lint")
        .and_then(|x| x.subcommand_matches("enable"))
        .unwrap();

    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;
    if !toml.is_empty() {
        mit_commit_message_lints::console::style::warning(
            "Warning: your config is overridden by a repository config file",
            None,
        );
    }

    let lints: Lints = subcommand
        .values_of("lint")
        .ok_or(LintNameNotGiven)?
        .collect::<Vec<_>>()
        .try_into()?;

    mit_commit_message_lints::lints::set_status(lints, &mut vcs, true)?;

    Ok(())
}
