use clap::ArgMatches;
use mit_commit_message_lints::{external, external::Vcs};

use crate::{current_dir, errors::GitMitConfigError, get_vcs};

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("relates-to")
        .filter(|subcommand| subcommand.subcommand_matches("template").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<(), GitMitConfigError> {
    let subcommand_args = matches
        .subcommand_matches("relates-to")
        .and_then(|x| x.subcommand_matches("template"))
        .unwrap();
    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let _toml = external::read_toml(current_dir)?;

    vcs.set_str(
        "mit.relate.template",
        subcommand_args.value_of("template").unwrap(),
    )?;

    Ok(())
}
