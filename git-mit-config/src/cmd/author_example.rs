use std::convert::TryInto;

use clap::{App, ArgMatches};
use miette::Result;
use mit_commit_message_lints::{console::style::to_be_piped, mit::Authors};

pub fn app<'help>() -> App<'help> {
    App::new("example").about("Print example mit toml file")
}

pub fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("mit")
        .filter(|subcommand| subcommand.subcommand_matches("example").is_some())
        .map(|_| run())
}

fn run() -> Result<()> {
    let example: String = Authors::example().try_into()?;
    to_be_piped(example.trim());

    Ok(())
}
