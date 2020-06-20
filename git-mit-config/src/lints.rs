use crate::errors::GitMitConfigError;
use crate::errors::GitMitConfigError::LintNameNotGiven;
use clap::ArgMatches;
use mit_commit_message_lints::external;
use mit_commit_message_lints::external::Vcs;
use mit_commit_message_lints::lints::set_status;
use mit_commit_message_lints::lints::Lint;
use mit_commit_message_lints::lints::Lints;
use std::convert::TryInto;
use std::path::PathBuf;

pub(crate) fn manage_lints(
    args: &ArgMatches,
    vcs: &mut dyn Vcs,
    current_dir: PathBuf,
) -> Result<(), GitMitConfigError> {
    if let Some(subcommand_args) = args.subcommand_matches("enable") {
        set_lint_status(vcs, &subcommand_args, true)
    } else if let Some(subcommand_args) = args.subcommand_matches("disable") {
        set_lint_status(vcs, &subcommand_args, false)
    } else if args.subcommand_matches("available").is_some() {
        let all_lints = Lints::new(Lint::iterator().collect());
        println!("{}", all_lints.names().join("\n"));
        Ok(())
    } else if args.subcommand_matches("enabled").is_some() {
        let toml = external::read_toml(current_dir)?;
        let lints = Lints::get_from_toml_or_else_vcs(&toml, vcs)?;
        println!("{}", lints.names().join("\n"));
        Ok(())
    } else if let Some(subcommand_args) = args.subcommand_matches("status") {
        let lints = get_selected_lints(&subcommand_args)?;

        let config = Lints::try_from_vcs(vcs)?;
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
        Err(GitMitConfigError::UnrecognisedLintCommand)
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

fn get_selected_lints(args: &ArgMatches) -> Result<Lints, GitMitConfigError> {
    let names: Vec<_> = args
        .values_of("lint")
        .ok_or_else(|| LintNameNotGiven)?
        .collect();
    let lints = names.try_into()?;

    Ok(lints)
}

pub fn set_lint_status(
    config: &mut dyn Vcs,
    args: &ArgMatches,
    status: bool,
) -> Result<(), GitMitConfigError> {
    set_status(get_selected_lints(&args)?, config, status).map_err(GitMitConfigError::from)
}
