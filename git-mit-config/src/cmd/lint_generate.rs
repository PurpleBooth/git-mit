use clap::ArgMatches;

use mit_commit_message_lints::external;
use mit_commit_message_lints::lints::Lints;

use crate::errors::GitMitConfigError;
use crate::{current_dir, get_vcs};
use std::convert::TryInto;

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("generate").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<(), GitMitConfigError> {
    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let input_toml = external::read_toml(current_dir)?;

    let output_toml: String =
        Lints::get_from_toml_or_else_vcs(&input_toml, &mut vcs)?.try_into()?;

    mit_commit_message_lints::console::to_be_piped(output_toml.trim());

    Ok(())
}
