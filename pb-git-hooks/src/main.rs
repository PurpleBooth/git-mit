use std::{
    convert::TryFrom,
    env,
    error::Error,
    fmt::{Display, Formatter},
    process,
};

use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches};
use git2::{Config, Repository};

use pb_commit_message_lints::{
    errors::PbCommitMessageLintsError,
    external::vcs::{Git2, Vcs},
    lints::{set_lint_status, Lints},
};

const LOCAL_SCOPE: &str = "local";
const LINT_NAME_ARGUMENT: &str = "lint";
const COMMAND_LINT: &str = "lint";
const COMMAND_LINT_ENABLE: &str = "enable";
const COMMAND_LINT_DISABLE: &str = "disable";
const SCOPE_ARGUMENT: &str = "scope";

fn display_err_and_exit<T>(error: &PbGitHooksError) -> T {
    eprintln!("{}", error);
    process::exit(1);
}

fn main() {
    let matches = app().get_matches();

    let current_dir = env::current_dir()
        .map_err(|error| PbGitHooksError::new_io("$PWD".into(), &error))
        .unwrap_or_else(|err| display_err_and_exit(&err));
    let git_config = match matches.value_of(SCOPE_ARGUMENT) {
        Some(LOCAL_SCOPE) => {
            Repository::discover(current_dir).and_then(|repo: Repository| repo.config())
        },
        _ => Config::open_default(),
    }
    .expect("Couldn't load any git config");
    let mut vcs = Git2::new(git_config);

    if let Some(value) = matches.subcommand_matches(COMMAND_LINT) {
        manage_lints(value, &mut vcs).unwrap_or_else(|err| display_err_and_exit(&err));
    }
}

fn app() -> App<'static, 'static> {
    let lint_argument = Arg::with_name(LINT_NAME_ARGUMENT)
        .help("The lint to enable")
        .required(true)
        .multiple(true)
        .min_values(1)
        .possible_values(
            Lints::iterator()
                .map(pb_commit_message_lints::lints::Lints::name)
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
                .short("s")
                .possible_values(&[LOCAL_SCOPE, "global"])
                .default_value(LOCAL_SCOPE),
        )
        .subcommand(
            App::new(COMMAND_LINT)
                .about("Manage active lints")
                .subcommand(
                    App::new(COMMAND_LINT_ENABLE)
                        .about("Enable a lint")
                        .arg(lint_argument.clone()),
                )
                .subcommand(
                    App::new(COMMAND_LINT_DISABLE)
                        .about("Disable a lint")
                        .arg(lint_argument.clone()),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

fn manage_lints(args: &ArgMatches, config: &mut dyn Vcs) -> Result<(), PbGitHooksError> {
    args.subcommand_matches(COMMAND_LINT_ENABLE)
        .map(|enable_args| (enable_args, true))
        .or_else(|| {
            args.subcommand_matches(COMMAND_LINT_DISABLE)
                .map(|disable_args| (disable_args, false))
        })
        .ok_or_else(|| PbGitHooksError::UnrecognisedLintCommand)
        .and_then(|(subcommand_args, enable)| {
            set_lint_status(
                &subcommand_args
                    .values_of(LINT_NAME_ARGUMENT)
                    .expect("Lint name not given")
                    .map(|name| {
                        Lints::try_from(name)
                            .map_err(PbGitHooksError::from)
                            .unwrap_or_else(|err| display_err_and_exit(&err))
                    })
                    .collect::<Vec<_>>(),
                config,
                enable,
            )
            .map_err(PbGitHooksError::from)
        })
}

#[derive(Debug)]
enum PbGitHooksError {
    UnrecognisedLintCommand,
    PbCommitMessageLintsError(PbCommitMessageLintsError),
    Io(String, String),
}

impl Display for PbGitHooksError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PbGitHooksError::UnrecognisedLintCommand => write!(
                f,
                "Unrecognised Lint command, only you may only enable or disable"
            ),
            PbGitHooksError::PbCommitMessageLintsError(error) => write!(f, "{}", error),
            PbGitHooksError::Io(file_source, error) => write!(
                f,
                "Failed to read git config from `{}`:\n{}",
                file_source, error
            ),
        }
    }
}

impl From<PbCommitMessageLintsError> for PbGitHooksError {
    fn from(from: PbCommitMessageLintsError) -> Self {
        PbGitHooksError::PbCommitMessageLintsError(from)
    }
}

impl Error for PbGitHooksError {}

impl PbGitHooksError {
    fn new_io(source: String, error: &std::io::Error) -> PbGitHooksError {
        PbGitHooksError::Io(source, format!("{}", error))
    }
}
