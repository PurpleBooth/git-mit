mod cli;
mod config;
mod errors;

use std::time::Duration;

use mit_commit_message_lints::console::exit_initial_not_matched_to_author;
use mit_commit_message_lints::console::exit_unparsable_author;
use mit_commit_message_lints::{
    external::Git2,
    mit::{set_commit_authors, Author, Authors},
};

use crate::cli::args::Args;
use crate::errors::GitMitError;
use crate::errors::GitMitError::NoAuthorInitialsProvided;
use std::convert::TryFrom;

fn main() -> Result<(), GitMitError> {
    let args: cli::args::Args = cli::app::app().get_matches().into();

    let mut git_config = Git2::try_from(Args::cwd()?)?;
    let file_authors = crate::config::author::load(&args);

    if let Err(error) = &file_authors {
        exit_unparsable_author(error);
    }

    let vcs_authors = &Authors::try_from(&git_config)?;
    let authors = file_authors?.merge(vcs_authors);

    let initials = args.initials().ok_or(NoAuthorInitialsProvided)?;
    let selected_authors = authors.get(&initials);
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
