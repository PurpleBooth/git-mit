use clap::ArgMatches;
use miette::Result;
use mit_commit_message_lints::{console::style::to_be_piped, mit::Authors};

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("mit")
        .filter(|subcommand| subcommand.subcommand_matches("example").is_some())
        .map(|_| {
            run();
            Ok(())
        })
}

fn run() {
    let example: String = Authors::example().into();
    to_be_piped(example.trim());
}
