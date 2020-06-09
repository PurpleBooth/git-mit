use crate::PbGitHooksError::LintNameNotGiven;
use crate::{
    PbGitHooksError, COMMAND_LINT_AVAILABLE, COMMAND_LINT_DISABLE, COMMAND_LINT_ENABLE,
    COMMAND_LINT_ENABLED, COMMAND_LINT_STATUS, LINT_NAME_ARGUMENT,
};
use clap::ArgMatches;
use pb_commit_message_lints::external::vcs::Vcs;
use pb_commit_message_lints::lints::{get_lint_configuration, Lints};
use std::convert::TryFrom;

pub(crate) fn manage_lints(args: &ArgMatches, config: &mut dyn Vcs) -> Result<(), PbGitHooksError> {
    if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_ENABLE) {
        set_lint_status(config, &subcommand_args, true)
    } else if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_DISABLE) {
        set_lint_status(config, &subcommand_args, false)
    } else if args.subcommand_matches(COMMAND_LINT_AVAILABLE).is_some() {
        let all_lints = Lints::iterator().collect::<Vec<_>>();
        println!("{}", get_lint_names(&all_lints).join("\n"));
        Ok(())
    } else if args.subcommand_matches(COMMAND_LINT_ENABLED).is_some() {
        let lints = get_lint_configuration(config)?;
        println!("{}", get_lint_names(&lints).join("\n"));
        Ok(())
    } else if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_STATUS) {
        let lints = get_selected_lints(&subcommand_args)?;

        let config = get_lint_configuration(config)?;
        let status = get_config_status(&lints, &config);
        let names = get_lint_names(&lints);

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

fn get_config_status<'a>(lints: &'a [Lints], config: &'a [Lints]) -> Vec<&'a str> {
    lints
        .iter()
        .map(|lint| {
            if config.contains(lint) {
                "enabled"
            } else {
                "disabled"
            }
        })
        .collect::<Vec<_>>()
}

fn get_lint_names(lints: &[Lints]) -> Vec<String> {
    lints
        .iter()
        .map(|lint| lint.name())
        .map(String::from)
        .collect()
}

fn get_selected_lints(args: &ArgMatches) -> Result<Vec<Lints>, PbGitHooksError> {
    let results = args
        .values_of(LINT_NAME_ARGUMENT)
        .ok_or_else(|| LintNameNotGiven)?
        .map(Lints::try_from)
        .collect::<Vec<_>>();

    let errors = results
        .iter()
        .filter(|result| result.is_err())
        .collect::<Vec<_>>();

    if let Some(Err(first_error)) = errors.first() {
        return Err(PbGitHooksError::from(first_error.clone()));
    }

    Ok(results.into_iter().flatten().collect())
}

pub fn set_lint_status(
    config: &mut dyn Vcs,
    args: &ArgMatches,
    status: bool,
) -> Result<(), PbGitHooksError> {
    pb_commit_message_lints::lints::set_lint_status(&get_selected_lints(&args)?, config, status)
        .map_err(PbGitHooksError::from)
}
