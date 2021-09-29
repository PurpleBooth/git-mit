use std::{convert::TryInto, env::current_dir, option::Option::None};

use clap::ArgMatches;
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::external;
use mit_lint::Lints;

use crate::{errors::GitMitConfigError::LintNameNotGiven, get_vcs};

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("disable").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<()> {
    let subcommand = matches
        .subcommand_matches("lint")
        .and_then(|x| x.subcommand_matches("disable"))
        .unwrap();

    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir().into_diagnostic()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;
    if !toml.is_empty() {
        mit_commit_message_lints::console::style::warning(
            "Warning: your config is overridden by a repository config file",
            None,
        );
    }

    let lint_names = subcommand.values_of("lint");
    if lint_names.is_none() {
        return Err(LintNameNotGiven.into());
    }

    let lints: Lints = lint_names
        .unwrap()
        .collect::<Vec<_>>()
        .try_into()
        .into_diagnostic()?;

    mit_commit_message_lints::lints::set_status(lints, &mut vcs, false)?;

    Ok(())
}
