mod cli;
mod config;
mod errors;

use std::{convert::TryFrom, time::Duration};

use mit_commit_message_lints::console::exit_initial_not_matched_to_author;
use mit_commit_message_lints::console::exit_unparsable_author;
use mit_commit_message_lints::mit::get_config_authors;
use mit_commit_message_lints::{
    external::Git2,
    mit::{set_commit_authors, Author},
};

use crate::cli::args::Args;
use crate::errors::GitMitError;
use crate::errors::GitMitError::NoAuthorInitialsProvided;

fn main() -> Result<(), GitMitError> {
    let args: cli::args::Args = cli::app::app().get_matches().into();
    let initials = args.initials().ok_or(NoAuthorInitialsProvided)?;

    let mut git_config = Git2::try_from(Args::cwd()?)?;
    let authors_file = crate::config::author::load(&args);

    if let Err(error) = &authors_file {
        exit_unparsable_author(error);
    }

    let all_authors = authors_file?.merge(&get_config_authors(&git_config)?);

    let selected_authors = all_authors.get(&initials);
    let initials_without_authors = find_initials_missing(initials, &selected_authors);

    if !initials_without_authors.is_empty() {
        exit_initial_not_matched_to_author(&initials_without_authors);
    }

    let authors = selected_authors.into_iter().flatten().collect::<Vec<_>>();
    set_commit_authors(
        &mut git_config,
        &authors,
        Duration::from_secs(args.timeout()? * 60),
    )?;

    Ok(())
}

fn find_initials_missing<'a>(
    authors_initials: Vec<&'a str>,
    selected_authors: &[Option<&Author>],
) -> Vec<&'a str> {
    selected_authors
        .iter()
        .zip(authors_initials)
        .filter_map(|(result, initial)| match result {
            None => Some(initial),
            Some(_) => None,
        })
        .collect()
}
