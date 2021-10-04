use std::convert::TryFrom;

use clap::ArgMatches;
use miette::Result;
use mit_commit_message_lints::{
    console::style::author_table,
    external::Git2,
    mit::{get_authors, AuthorArgs, Authors},
};

use crate::current_dir;

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("mit")
        .filter(|subcommand| {
            subcommand.subcommand_matches("generate").is_some()
                || subcommand.subcommand_matches("available").is_some()
        })
        .map(|_| run(matches))
}

pub struct Args {
    matches: ArgMatches,
}

impl Args {
    fn is_generate_command(&self) -> bool {
        self.matches
            .subcommand_matches("mit")
            .and_then(|matches| matches.subcommand_matches("generate"))
            .is_some()
    }

    fn normalised_args(&self) -> Option<&ArgMatches> {
        self.matches.subcommand_matches("mit").and_then(|matches| {
            matches
                .subcommand_matches("generate")
                .or_else(|| matches.subcommand_matches("available"))
        })
    }
}

impl From<&ArgMatches> for Args {
    fn from(matches: &ArgMatches) -> Self {
        Args {
            matches: matches.clone(),
        }
    }
}

impl AuthorArgs for Args {
    fn author_command(&self) -> Option<&str> {
        self.normalised_args()
            .and_then(|matches| matches.value_of("command"))
    }

    fn author_file(&self) -> Option<&str> {
        self.normalised_args()
            .and_then(|matches| matches.value_of("file"))
    }
}

fn run(matches: &ArgMatches) -> Result<()> {
    let args = Args::from(matches);
    let file_authors = get_authors(&args)?;
    let git_config = current_dir().and_then(Git2::try_from)?;
    let authors = file_authors.merge(&Authors::try_from(&git_config)?);

    let output: String = if args.is_generate_command() {
        to_toml(authors)
    } else {
        author_table(&authors)
    };

    mit_commit_message_lints::console::style::to_be_piped(&output);
    Ok(())
}

fn to_toml(authors: Authors) -> String {
    String::from(authors).trim().to_string()
}
