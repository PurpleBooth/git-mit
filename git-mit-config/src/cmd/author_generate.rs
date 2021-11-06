use std::convert::TryFrom;

use clap::{App, Arg, ArgMatches};
use miette::Result;
use mit_commit_message_lints::{
    console::style::author_table,
    external::Git2,
    mit::{get_authors, AuthorArgs, Authors},
};

use crate::current_dir;

const APP_FILE: &str = "file";
const APP_FILE_SHORT: char = 'c';
const APP_FILE_LONG: &str = "config";
const APP_FILE_ABOUT: &str = "Path to a file where mit initials, emails and names can be found";
const APP_FILE_ENV: &str = "GIT_MIT_AUTHORS_CONFIG";
const APP_FILE_DEFAULT: &str = "$HOME/.config/git-mit/mit.toml";

const APP_COMMAND: &str = "command";
const APP_COMMAND_SHORT: char = 'e';
const APP_COMMAND_LONG: &str = "exec";
const APP_COMMAND_ABOUT: &str = "Execute a command to generate the mit configuration, stdout will be captured and used instead of the file, if both this and the file is present, this takes precedence";
const APP_COMMAND_ENV: &str = "GIT_MIT_AUTHORS_EXEC";

pub fn app_generate<'help>() -> App<'help> {
    App::new("generate")
        .arg(
            Arg::new(APP_FILE)
                .short(APP_FILE_SHORT)
                .long(APP_FILE_LONG)
                .about(APP_FILE_ABOUT)
                .env(APP_FILE_ENV)
                .default_value(APP_FILE_DEFAULT)
                .takes_value(true),
        )
        .arg(
            Arg::new(APP_COMMAND)
                .short(APP_COMMAND_SHORT)
                .long(APP_COMMAND_LONG)
                .about(APP_COMMAND_ABOUT)
                .env(APP_COMMAND_ENV)
                .takes_value(true),
        )
        .about("Generate a file version of available authors")
}

pub fn app_available<'help>() -> App<'help> {
    App::new("available")
        .arg(
            Arg::new(APP_FILE)
                .short(APP_FILE_SHORT)
                .long(APP_FILE_LONG)
                .about(APP_FILE_ABOUT)
                .env(APP_FILE_ENV)
                .default_value(APP_FILE_DEFAULT)
                .takes_value(true),
        )
        .arg(
            Arg::new(APP_COMMAND)
                .short(APP_COMMAND_SHORT)
                .long(APP_COMMAND_LONG)
                .about(APP_COMMAND_ABOUT)
                .env(APP_COMMAND_ENV)
                .takes_value(true),
        )
        .about("List available authors")
}

pub fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
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
        Self {
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
        to_toml(authors)?
    } else {
        author_table(&authors)
    };

    mit_commit_message_lints::console::style::to_be_piped(&output);
    Ok(())
}

fn to_toml(authors: Authors<'_>) -> Result<String> {
    Ok(String::try_from(authors)?.trim().to_string())
}
