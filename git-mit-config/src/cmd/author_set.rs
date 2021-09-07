use std::env::current_dir;

use clap::ArgMatches;
use mit_commit_message_lints::mit::{set_config_authors, Author};

use crate::{errors::GitMitConfigError, get_vcs};

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("mit")
        .filter(|subcommand| subcommand.subcommand_matches("set").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<(), GitMitConfigError> {
    let subcommand = matches
        .subcommand_matches("mit")
        .and_then(|x| x.subcommand_matches("set"))
        .unwrap();

    let initial = subcommand.value_of("initial").unwrap();
    let name = subcommand.value_of("name").unwrap();
    let email = subcommand.value_of("email").unwrap();
    let signingkey = subcommand.value_of("signingkey");

    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    set_config_authors(&mut vcs, initial, &Author::new(name, email, signingkey))?;

    Ok(())
}
