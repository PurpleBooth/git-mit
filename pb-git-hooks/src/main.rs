use std::{
    convert::TryFrom,
    env,
    error::Error,
    fmt::{Display, Formatter},
    process,
};

use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches};
use git2::{Config, Repository};

use itertools::Itertools;
use pb_commit_message_lints::{
    author::entities::Authors,
    errors::PbCommitMessageLintsError,
    external::vcs::{Git2, Vcs},
    lints::{get_lint_configuration, Lints},
};
use std::convert::TryInto;

const LOCAL_SCOPE: &str = "local";
const LINT_NAME_ARGUMENT: &str = "lint";
const COMMAND_LINT: &str = "lint";
const COMMAND_AUTHORS: &str = "authors";
const COMMAND_AUTHORS_EXAMPLE: &str = "example";
const COMMAND_LINT_AVAILABLE: &str = "available";
const COMMAND_LINT_ENABLE: &str = "enable";
const COMMAND_LINT_DISABLE: &str = "disable";
const SCOPE_ARGUMENT: &str = "scope";
const COMMAND_LINT_ENABLED: &str = "enabled";
const COMMAND_LINT_STATUS: &str = "status";

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
    if let Some(subcommand) = matches.subcommand_matches(COMMAND_AUTHORS) {
        if subcommand
            .subcommand_matches(COMMAND_AUTHORS_EXAMPLE)
            .is_some()
        {
            let example: String = Authors::example()
                .try_into()
                .map_err(PbGitHooksError::from)
                .unwrap_or_else(|err| display_err_and_exit(&err));
            println!("{}", example)
        }
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

fn manage_lints(args: &ArgMatches, config: &mut dyn Vcs) -> Result<(), PbGitHooksError> {
    if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_ENABLE) {
        set_lint_status(config, &subcommand_args, true)
    } else if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_DISABLE) {
        set_lint_status(config, &subcommand_args, false)
    } else if args.subcommand_matches(COMMAND_LINT_AVAILABLE).is_some() {
        println!(
            "{}",
            Lints::iterator()
                .map(pb_commit_message_lints::lints::Lints::name)
                .join("\n")
        );
        Ok(())
    } else if args.subcommand_matches(COMMAND_LINT_ENABLED).is_some() {
        let lints = get_lint_configuration(config)?;
        println!(
            "{}",
            lints
                .into_iter()
                .map(pb_commit_message_lints::lints::Lints::name)
                .join("\n")
        );
        Ok(())
    } else if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_STATUS) {
        let lints = &subcommand_args
            .values_of(LINT_NAME_ARGUMENT)
            .expect("Lint name not given")
            .map(|name| {
                Lints::try_from(name)
                    .map_err(PbGitHooksError::from)
                    .unwrap_or_else(|err| display_err_and_exit(&err))
            })
            .collect::<Vec<_>>();

        let user_status = get_lint_configuration(config)?;
        println!(
            "{}",
            lints
                .iter()
                .map(|lint| format!(
                    "{}\t{}",
                    lint.name(),
                    if user_status.contains(lint) {
                        "enabled"
                    } else {
                        "disabled"
                    }
                ))
                .collect_vec()
                .join("\n")
        );
        Ok(())
    } else {
        Err(PbGitHooksError::UnrecognisedLintCommand)
    }
}

fn set_lint_status(
    config: &mut dyn Vcs,
    subcommand_args: &ArgMatches,
    status: bool,
) -> Result<(), PbGitHooksError> {
    pb_commit_message_lints::lints::set_lint_status(
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
        status,
    )
    .map_err(PbGitHooksError::from)
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
                "Unrecognised Lint command, only you may only enable or disable, or list \
                 available lints"
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
