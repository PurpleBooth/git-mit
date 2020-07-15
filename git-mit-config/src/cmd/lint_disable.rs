use std::convert::TryInto;
use std::env::current_dir;

use clap::ArgMatches;

use mit_commit_message_lints::external;
use mit_commit_message_lints::lints::Lints;

use crate::errors::GitMitConfigError;
use crate::errors::GitMitConfigError::LintNameNotGiven;
use crate::get_vcs;

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("disable").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<(), GitMitConfigError> {
    let subcommand = matches
        .subcommand_matches("lint")
        .and_then(|x| x.subcommand_matches("disable"))
        .unwrap();

    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;
    if !toml.is_empty() {
        mit_commit_message_lints::console::style::warning(
            "Warning: your config is overridden by a repository config file",
        )
    }

    let lints: Lints = subcommand
        .values_of("lint")
        .ok_or_else(|| LintNameNotGiven)?
        .collect::<Vec<_>>()
        .try_into()?;

    mit_commit_message_lints::lints::set_status(lints, &mut vcs, false)?;

    Ok(())
}
