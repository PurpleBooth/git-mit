use clap::ArgMatches;

use mit_commit_message_lints::lints::{Lint, Lints};

use crate::errors::GitMitConfigError;

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("available").is_some())
        .map(|_| run())
}

fn run() -> Result<(), GitMitConfigError> {
    let all_lints = Lints::new(Lint::iterator().collect());
    println!("{}", all_lints.names().join("\n"));
    Ok(())
}
