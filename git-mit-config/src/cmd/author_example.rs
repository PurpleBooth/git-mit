use std::convert::TryInto;

use clap::ArgMatches;

use mit_commit_message_lints::author::entities::Authors;

use crate::errors::GitMitConfigError;

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("mit")
        .filter(|subcommand| subcommand.subcommand_matches("example").is_some())
        .map(|_| run())
}

fn run() -> Result<(), GitMitConfigError> {
    let example: String = Authors::example().try_into()?;
    println!("{}", example);

    Ok(())
}
