use std::convert::TryInto;

use clap::{Arg, ArgMatches, Command};
use miette::Result;
use mit_commit_message_lints::{external, lints::read_from_toml_or_else_vcs};
use mit_lint::Lints;

use crate::{current_dir, errors::GitMitConfigError::LintNameNotGiven, get_vcs};

pub fn cli<'help>(lint_names: &'help [&'help str]) -> Command<'help> {
    Command::new("status")
        .arg(
            Arg::new("scope")
                .long("scope")
                .short('s')
                .possible_values(&["local", "global"])
                .default_value("local"),
        )
        .about("Get status of a lint")
        .arg(
            Arg::new("lint")
                .help("The lint to enable")
                .required(true)
                .multiple_values(true)
                .min_values(1)
                .possible_values(lint_names)
                .clone(),
        )
}

pub fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("status").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<()> {
    let subcommand = matches
        .subcommand_matches("lint")
        .and_then(|matches| matches.subcommand_matches("status"))
        .unwrap();
    let is_local = Some("local") == subcommand.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;

    let lints = get_selected_lints(subcommand)?;
    let config = read_from_toml_or_else_vcs(&toml, &mut vcs)?;

    mit_commit_message_lints::console::style::lint_table(&lints, &config);

    Ok(())
}

fn get_selected_lints(args: &ArgMatches) -> Result<Lints> {
    let lint_names = args.values_of("lint");
    if lint_names.is_none() {
        return Err(LintNameNotGiven.into());
    }

    Ok(lint_names.unwrap().collect::<Vec<_>>().try_into()?)
}
