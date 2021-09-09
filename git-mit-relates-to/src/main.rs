use std::{convert::TryFrom, env};

use cli::{app, args::Args};
use git2::Repository;
use mit_commit_message_lints::{
    console::style,
    external::Git2,
    relates::{entities::RelateTo, vcs::set_relates_to},
};

mod cli;
mod errors;

fn main() -> Result<(), errors::GitRelatesTo> {
    let args: Args = app::app().get_matches().into();

    let relates_to = args.issue_number()?;

    if repo_present() && !is_hook_present() {
        not_setup_warning();
    };

    let current_dir = env::current_dir()?;
    let mut vcs = Git2::try_from(current_dir)?;
    set_relates_to(&mut vcs, &RelateTo::new(relates_to), args.timeout()?)?;

    Ok(())
}

fn not_setup_warning() {
    style::warning("Hooks not found in this repository, your commits won't contain trailers, and lints will not be checked", Some("git mit-install\n\nwill fix this"));
}

fn is_hook_present() -> bool {
    env::current_dir()
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
    env::current_dir()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .is_some()
}
