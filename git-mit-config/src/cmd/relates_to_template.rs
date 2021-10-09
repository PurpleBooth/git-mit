use clap::ArgMatches;
use miette::Result;
use mit_commit_message_lints::external::Vcs;

use crate::{current_dir, get_vcs};

pub fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("relates-to")
        .filter(|subcommand| subcommand.subcommand_matches("template").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<()> {
    let subcommand = matches
        .subcommand_matches("relates-to")
        .and_then(|x| x.subcommand_matches("template"))
        .unwrap();
    let is_local = Some("local") == subcommand.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;

    vcs.set_str(
        "mit.relate.template",
        subcommand.value_of("template").unwrap(),
    )?;

    Ok(())
}
