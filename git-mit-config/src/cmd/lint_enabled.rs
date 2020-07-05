use clap::ArgMatches;

use mit_commit_message_lints::external;
use mit_commit_message_lints::lints::Lints;

use crate::errors::GitMitConfigError;
use crate::{current_dir, get_vcs};
use comfy_table::Table;

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("lint")
        .filter(|subcommand| subcommand.subcommand_matches("enabled").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<(), GitMitConfigError> {
    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;

    let lints = Lints::get_from_toml_or_else_vcs(&toml, &mut vcs)?;
    let mut table = Table::new();
    table.set_header(vec!["Lint", "Status"]);

    let rows: Table = lints.into_iter().fold(table, |mut table, lint| {
        table.add_row(vec![lint.name(), "enabled"]);
        table
    });

    println!("{}", rows);

    Ok(())
}
