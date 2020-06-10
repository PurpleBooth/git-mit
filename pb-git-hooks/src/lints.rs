use crate::PbGitHooksError::LintNameNotGiven;
use crate::{
    PbGitHooksError, COMMAND_LINT_AVAILABLE, COMMAND_LINT_DISABLE, COMMAND_LINT_ENABLE,
    COMMAND_LINT_ENABLED, COMMAND_LINT_STATUS, LINT_NAME_ARGUMENT,
};
use clap::ArgMatches;
use pb_commit_message_lints::external::vcs::Vcs;
use pb_commit_message_lints::lints;

pub(crate) fn manage_lints(args: &ArgMatches, config: &mut dyn Vcs) -> Result<(), PbGitHooksError> {
    if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_ENABLE) {
        set_lint_status(config, &subcommand_args, true)
    } else if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_DISABLE) {
        set_lint_status(config, &subcommand_args, false)
    } else if args.subcommand_matches(COMMAND_LINT_AVAILABLE).is_some() {
        let all_lints = lints::Lints::iterator().collect::<Vec<_>>();
        println!("{}", lints::Lints::convert_to_names(&all_lints).join("\n"));
        Ok(())
    } else if args.subcommand_matches(COMMAND_LINT_ENABLED).is_some() {
        let lints = lints::get_lint_configuration(config)?;
        println!("{}", lints::Lints::convert_to_names(&lints).join("\n"));
        Ok(())
    } else if let Some(subcommand_args) = args.subcommand_matches(COMMAND_LINT_STATUS) {
        let lints = get_selected_lints(&subcommand_args)?;

        let config = lints::get_lint_configuration(config)?;
        let status = get_config_status(&lints, &config);
        let names = lints::Lints::convert_to_names(&lints);

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

fn get_config_status<'a>(lints: &'a [lints::Lints], config: &'a [lints::Lints]) -> Vec<&'a str> {
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

fn get_selected_lints(args: &ArgMatches) -> Result<Vec<lints::Lints>, PbGitHooksError> {
    let results = lints::Lints::from_names(
        args.values_of(LINT_NAME_ARGUMENT)
            .ok_or_else(|| LintNameNotGiven)?
            .collect(),
    );

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
