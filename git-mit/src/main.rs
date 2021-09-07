mod cli;
mod config;
mod errors;

use std::{convert::TryFrom, time::Duration};

use mit_commit_message_lints::{
    console::{exit_initial_not_matched_to_author, exit_unparsable_author},
    external::Git2,
    mit::{set_commit_authors, Authors},
};

use crate::{cli::args::Args, errors::GitMitError};

fn main() -> Result<(), GitMitError> {
    let args: cli::args::Args = cli::app::app().get_matches().into();

    let mut git_config = Git2::try_from(Args::cwd()?)?;
    let file_authors = crate::config::author::load(&args);

    if let Err(error) = &file_authors {
        exit_unparsable_author(error);
    }

    let authors = file_authors?.merge(&Authors::try_from(&git_config)?);
    let initials = args.initials()?;
    let missing = authors.missing_initials(initials.clone());

    if !missing.is_empty() {
        exit_initial_not_matched_to_author(&missing);
    }

    set_commit_authors(
        &mut git_config,
        &authors.get(&initials),
        Duration::from_secs(args.timeout()? * 60),
    )?;

    Ok(())
}
