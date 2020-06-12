use crate::PbGitHooksError::LintNameNotGiven;
use crate::{
    PbGitHooksError, COMMAND_LINT_AVAILABLE, COMMAND_LINT_DISABLE, COMMAND_LINT_ENABLE,
    COMMAND_LINT_ENABLED, COMMAND_LINT_STATUS, LINT_NAME_ARGUMENT,
};
use clap::ArgMatches;
use pb_commit_message_lints::external::vcs::Vcs;
use pb_commit_message_lints::lints;
use pb_commit_message_lints::lints::collection::Lints;
use pb_commit_message_lints::lints::get_lint_configuration;
use std::convert::TryInto;

pub(crate) fn manage_lints(args: &ArgMatches, config: &mut dyn Vcs) -> Result<(), PbGitHooksError> {
    if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_ENABLE) {
        set_lint_status(config, &subcommand_args, true)
    } else if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_DISABLE) {
        set_lint_status(config, &subcommand_args, false)
    } else if args.subcommand_matches(COMMAND_LINT_AVAILABLE).is_some() {
        let all_lints = Lints::new(lints::Lint::iterator().collect());
        println!("{}", all_lints.names().join("\n"));
        Ok(())
    } else if args.subcommand_matches(COMMAND_LINT_ENABLED).is_some() {
        let lints: Lints = get_lint_configuration(config)?;
        println!("{}", lints.names().join("\n"));
        Ok(())
    } else if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_STATUS) {
        let lints = get_selected_lints(&subcommand_args)?;

        let config = get_lint_configuration(config)?;
        let status = get_config_status(lints.clone(), config);
        let names = lints.names();

        println!(
            "{}",
            names
                .iter()
                .zip(status)
                .map(|(name, status)| format!("{}\t{}", name, status))
                .collect::<Vec<_>>()
                .join("\n")
        );
        Ok(())
    } else {
        Err(PbGitHooksError::UnrecognisedLintCommand)
    }
}

fn get_config_status<'a>(lints: Lints, config: Lints) -> Vec<&'a str> {
    let enabled_lints: Vec<_> = config.into_iter().collect();

    lints
        .into_iter()
        .map(|lint| {
            if enabled_lints.contains(&lint) {
                "enabled"
            } else {
                "disabled"
            }
        })
        .collect::<Vec<_>>()
}

fn get_selected_lints(args: &ArgMatches) -> Result<lints::collection::Lints, PbGitHooksError> {
    let names: Vec<_> = args
        .values_of(LINT_NAME_ARGUMENT)
        .ok_or_else(|| LintNameNotGiven)?
        .collect();
    let lints = names.try_into()?;

    Ok(lints)
}

pub fn set_lint_status(
    config: &mut dyn Vcs,
    args: &ArgMatches,
    status: bool,
) -> Result<(), PbGitHooksError> {
    pb_commit_message_lints::lints::set_lint_status(get_selected_lints(&args)?, config, status)
        .map_err(PbGitHooksError::from)
}
