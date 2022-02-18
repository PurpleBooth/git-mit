//! the git mit-relates to binary

#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use std::{convert::TryFrom, env, io::stdout};

use cli::{app, args::Args};
use git2::Repository;
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    console::{completion::print_completions, error_handling::miette_install, style},
    external::Git2,
    relates::{set_relates_to, RelateTo},
};

mod cli;
mod errors;

fn main() -> Result<()> {
    miette_install();

    let mut app = app::cli();
    let args: Args = app.clone().get_matches().into();

    // Simply print and exit if completion option is given.
    if let Some(completion) = args.completion() {
        print_completions(&mut stdout(), &mut app, completion);

        std::process::exit(0);
    }

    let relates_to = args.issue_number()?;

    if repo_present() && !is_hook_present() {
        not_setup_warning();
    };

    let current_dir = env::current_dir().into_diagnostic()?;
    let mut vcs = Git2::try_from(current_dir)?;
    set_relates_to(&mut vcs, &RelateTo::from(relates_to), args.timeout()?)?;

    Ok(())
}

fn not_setup_warning() {
    style::warning("Hooks not found in this repository, your commits won't contain trailers, and lints will not be checked", Some("`git mit-install` `will fix this"));
}

fn is_hook_present() -> bool {
    env::current_dir()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .map(|repo| repo.path().join("hooks").join("commit-msg"))
        .filter(|path_buf| match path_buf.canonicalize().ok() {
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
