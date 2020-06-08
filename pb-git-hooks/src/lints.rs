use crate::{
    display_err_and_exit, PbGitHooksError, COMMAND_LINT_AVAILABLE, COMMAND_LINT_DISABLE,
    COMMAND_LINT_ENABLE, COMMAND_LINT_ENABLED, COMMAND_LINT_STATUS, LINT_NAME_ARGUMENT,
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
        println!(
            "{}",
            Lints::iterator()
                .map(pb_commit_message_lints::lints::Lints::name)
                .collect::<Vec<_>>()
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
                .collect::<Vec<_>>()
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
                .collect::<Vec<_>>()
                .join("\n")
        );
        Ok(())
    } else {
        Err(PbGitHooksError::UnrecognisedLintCommand)
    }
}

pub fn set_lint_status(
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
