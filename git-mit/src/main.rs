mod cli;
mod config;
mod errors;

use std::{convert::TryFrom, option::Option::None, time::Duration};

use git2::Repository;
use mit_commit_message_lints::{
    console::{exit_initial_not_matched_to_author, exit_unparsable_author, style},
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
    };

    let authors = file_authors?.merge(&Authors::try_from(&git_config)?);
    let initials = args.initials()?;

    if repo_present() && !is_hook_present() {
        not_setup_warning();
    };

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

fn not_setup_warning() {
    style::warning("Hooks not found in this repository, your commits won't contain trailers, and lints will not be checked", Some("git mit-install\n\nwill fix this"));
}

fn is_hook_present() -> bool {
    Args::cwd()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .map(|x| x.path().join("hooks").join("commit-msg"))
        .filter(|x| match x.canonicalize().ok() {
            None => false,
            Some(path) => path.to_string_lossy().contains("mit-commit-msg"),
        })
        .is_some()
}

fn repo_present() -> bool {
    Args::cwd()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .is_some()
}
