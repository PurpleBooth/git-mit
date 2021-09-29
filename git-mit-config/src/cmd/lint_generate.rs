use std::convert::TryInto;

use clap::ArgMatches;
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{external, lints::read_from_toml_or_else_vcs};

use crate::{current_dir, get_vcs};

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("generate").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<()> {
    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let input_toml = external::read_toml(current_dir)?;

    let output_toml: String = read_from_toml_or_else_vcs(&input_toml, &mut vcs)?
        .try_into()
        .into_diagnostic()?;

    mit_commit_message_lints::console::to_be_piped(output_toml.trim());

    Ok(())
}
