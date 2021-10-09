use clap::ArgMatches;
use miette::Result;
use mit_commit_message_lints::{external, lints::read_from_toml_or_else_vcs};

use crate::{current_dir, get_vcs};

pub fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("enabled").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<()> {
    let subcommand = matches
        .subcommand_matches("lint")
        .and_then(|matches| matches.subcommand_matches("enabled"))
        .unwrap();
    let is_local = Some("local") == subcommand.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;

    let lints = read_from_toml_or_else_vcs(&toml, &mut vcs)?;
    mit_commit_message_lints::console::style::lint_table(&lints, &lints);

    Ok(())
}
