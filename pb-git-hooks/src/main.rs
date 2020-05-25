use std::{convert::TryFrom, env, error::Error};

use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use enum_iterator::IntoEnumIterator;
use git2::{Config, Repository};

use pb_commit_message_lints::{
    external::vcs::{Git2, Vcs},
    lints::{set_lint_status, Lints},
};

const LOCAL_SCOPE: &str = "local";
const LINT_NAME_ARGUMENT: &str = "lint";
const COMMAND_LINT: &str = "lint";
const COMMAND_LINT_ENABLE: &str = "enable";
const COMMAND_LINT_DISABLE: &str = "disable";
const SCOPE_ARGUMENT: &str = "scope";

fn main() -> Result<(), Box<dyn Error>> {
    let possible_lints: Vec<&str> = Lints::into_enum_iter().map(|lint| lint.into()).collect();
    let lint_argument = Arg::with_name(LINT_NAME_ARGUMENT)
        .help("The lint to enable")
        .required(true)
        .multiple(true)
        .possible_values(&possible_lints);
    let matches = App::new(env!("CARGO_PKG_NAME"))
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
        .get_matches();

    let current_dir = env::current_dir().unwrap();
    let git_config = match matches.value_of(SCOPE_ARGUMENT) {
        Some(LOCAL_SCOPE) => Repository::discover(current_dir).and_then(|x: Repository| x.config()),
        _ => Config::open_default(),
    }
    .expect("Couldn't load any git config");
    let mut vcs = Git2::new(git_config);

    if let Some(value) = matches.subcommand_matches(COMMAND_LINT) {
        return manage_lints(value, &mut vcs);
    }

    Ok(())
}

fn manage_lints(args: &ArgMatches, config: &mut dyn Vcs) -> Result<(), Box<dyn Error>> {
    set_lint_status(
        &args
            .values_of(LINT_NAME_ARGUMENT)
            .unwrap()
            .map(|name| Lints::try_from(name).unwrap())
            .collect::<Vec<Lints>>(),
        config,
        args.subcommand_matches(COMMAND_LINT_ENABLE).is_some(),
    )
}
