use std::convert::TryInto;

use clap::{Arg, ArgMatches, Command};
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    console::style::to_be_piped,
    external,
    lints::read_from_toml_or_else_vcs,
};

use crate::{current_dir, get_vcs};

pub fn cli<'help>() -> Command<'help> {
    Command::new("generate")
        .arg(
            Arg::new("scope")
                .long("scope")
                .short('s')
                .possible_values(["local", "global"])
                .default_value("local"),
        )
        .about("Generate the config file for your current settings")
}

pub fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("generate").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<()> {
    let subcommand = matches
        .subcommand_matches("lint")
        .and_then(|matches| matches.subcommand_matches("generate"))
        .unwrap();
    let is_local = Some("local") == subcommand.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let input_toml = external::read_toml(current_dir)?;

    let output_toml: String = read_from_toml_or_else_vcs(&input_toml, &mut vcs)?
        .try_into()
        .into_diagnostic()?;

    to_be_piped(output_toml.trim());

    Ok(())
}
