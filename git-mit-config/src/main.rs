use std::{
    env,
    error::Error,
    fmt::{Display, Formatter},
};

use clap::{crate_authors, crate_version, App, AppSettings, Arg};
use git2::{Config, Repository};

use crate::lints::manage_lints;
use mit_commit_message_lints::lints::Lint;
use mit_commit_message_lints::{
    author::entities::Authors, errors::MitCommitMessageLintsError, external::vcs::Git2,
};
use std::convert::TryInto;

const LOCAL_SCOPE: &str = "local";
const LINT_NAME_ARGUMENT: &str = "lint";
const COMMAND_LINT: &str = "lint";
const COMMAND_AUTHORS: &str = "mit";
const COMMAND_AUTHORS_EXAMPLE: &str = "example";
const COMMAND_LINT_AVAILABLE: &str = "available";
const COMMAND_LINT_ENABLE: &str = "enable";
const COMMAND_LINT_DISABLE: &str = "disable";
const SCOPE_ARGUMENT: &str = "scope";
const COMMAND_LINT_ENABLED: &str = "enabled";
const COMMAND_LINT_STATUS: &str = "status";

fn main() -> Result<(), GitMitConfigError> {
    let matches = app().get_matches();

    if let Some(subcommand) = matches.subcommand_matches(COMMAND_AUTHORS) {
        if subcommand
            .subcommand_matches(COMMAND_AUTHORS_EXAMPLE)
            .is_some()
        {
            let example: String = Authors::example().try_into()?;
            println!("{}", example)
        };
        Ok(())
    } else if let Some(value) = matches.subcommand_matches(COMMAND_LINT) {
        let current_dir =
            env::current_dir().map_err(|error| GitMitConfigError::new_io("$PWD".into(), &error))?;

        let git_config = match matches.value_of(SCOPE_ARGUMENT) {
            Some(LOCAL_SCOPE) => {
                Repository::discover(current_dir).and_then(|repo: Repository| repo.config())
            }
            _ => Config::open_default(),
        }?;

        let mut vcs = Git2::new(git_config);
        manage_lints(value, &mut vcs)
    } else {
        Ok(())
    }
}

fn app() -> App<'static> {
    let lint_argument = Arg::with_name(LINT_NAME_ARGUMENT)
        .about("The lint to enable")
        .required(true)
        .multiple(true)
        .min_values(1)
        .possible_values(
            Lint::iterator()
                .map(mit_commit_message_lints::lints::Lint::name)
                .collect::<Vec<_>>()
                .as_slice(),
        );
    App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(SCOPE_ARGUMENT)
                .long("scope")
                .short('s')
                .possible_values(&[LOCAL_SCOPE, "global"])
                .default_value(LOCAL_SCOPE),
        )
        .subcommand(
            App::new(COMMAND_LINT)
                .about("Manage active lints")
                .subcommand(App::new(COMMAND_LINT_AVAILABLE).about("List the available lints"))
                .subcommand(App::new(COMMAND_LINT_ENABLED).about("List the enabled lints"))
                .subcommand(
                    App::new(COMMAND_LINT_STATUS)
                        .about("Get status of a lint")
                        .arg(lint_argument.clone()),
                )
                .subcommand(
                    App::new(COMMAND_LINT_ENABLE)
                        .about("Enable a lint")
                        .arg(lint_argument.clone()),
                )
                .subcommand(
                    App::new(COMMAND_LINT_DISABLE)
                        .about("Disable a lint")
                        .arg(lint_argument.clone()),
                )
                .setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .subcommand(
            App::new(COMMAND_AUTHORS)
                .about("Manage author configuration")
                .subcommand(
                    App::new(COMMAND_AUTHORS_EXAMPLE).about("Print example author yaml file"),
                )
                .setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

mod lints;

#[derive(Debug)]
pub enum GitMitConfigError {
    ConfigIoGit2Error(String),
    UnrecognisedLintCommand,
    LintNameNotGiven,
    MitCommitMessageLintsError(MitCommitMessageLintsError),
    Io(String, String),
}

impl Display for GitMitConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitMitConfigError::UnrecognisedLintCommand => write!(
                f,
                "Unrecognised Lint command, only you may only enable or disable, or list \
                 available lints"
            ),
            GitMitConfigError::LintNameNotGiven => write!(f, "Please specify a lint"),
            GitMitConfigError::MitCommitMessageLintsError(error) => write!(f, "{}", error),
            GitMitConfigError::Io(file_source, error) => write!(
                f,
                "Failed to read git config from `{}`:\n{}",
                file_source, error
            ),
            GitMitConfigError::ConfigIoGit2Error(error) => {
                write!(f, "Failed to load a git config: {}", error)
            }
        }
    }
}

impl From<git2::Error> for GitMitConfigError {
    fn from(error: git2::Error) -> Self {
        GitMitConfigError::ConfigIoGit2Error(format!("{}", error))
    }
}

impl From<MitCommitMessageLintsError> for GitMitConfigError {
    fn from(from: MitCommitMessageLintsError) -> Self {
        GitMitConfigError::MitCommitMessageLintsError(from)
    }
}

impl Error for GitMitConfigError {}

impl GitMitConfigError {
    fn new_io(source: String, error: &std::io::Error) -> GitMitConfigError {
        GitMitConfigError::Io(source, format!("{}", error))
    }
}
