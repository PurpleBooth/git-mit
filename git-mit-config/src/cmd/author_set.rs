use std::env::current_dir;

use clap::{App, Arg, ArgMatches};
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::mit::{set_config_authors, Author};

use crate::get_vcs;

pub fn app<'help>() -> App<'help> {
    App::new("set")
        .arg(
            Arg::new("scope")
                .long("scope")
                .short('s')
                .possible_values(&["local", "global"])
                .default_value("local"),
        )
        .arg(
            Arg::new("initial")
                .about("Initial of the mit to update or add")
                .required(true),
        )
        .arg(
            Arg::new("name")
                .about("Name to use for the mit in format \"Forename Surname\"")
                .required(true),
        )
        .arg(
            Arg::new("email")
                .about("Email to use for the mit")
                .required(true),
        )
        .arg(
            Arg::new("signingkey")
                .about("Signing key to use for this user")
                .required(false),
        )
        .about("Update or add an initial in the mit configuration")
}

pub fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("mit")
        .filter(|subcommand| subcommand.subcommand_matches("set").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<()> {
    let subcommand = matches
        .subcommand_matches("mit")
        .and_then(|x| x.subcommand_matches("set"))
        .unwrap();

    let initial = subcommand.value_of("initial").unwrap();
    let name = subcommand.value_of("name").unwrap();
    let email = subcommand.value_of("email").unwrap();
    let signingkey = subcommand.value_of("signingkey");

    let is_local = Some("local") == subcommand.value_of("scope");
    let current_dir = current_dir().into_diagnostic()?;
    let mut vcs = get_vcs(is_local, &current_dir)?;
    set_config_authors(
        &mut vcs,
        initial,
        &Author::new(
            name.into(),
            email.into(),
            signingkey.map(std::convert::Into::into),
        ),
    )?;

    Ok(())
}
