//! the git mit-relates to binary

#![warn(clippy::nursery)]
#![deny(
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic,
    non_fmt_panics
)]
#![allow(clippy::multiple_crate_versions)]

use std::{convert::TryFrom, env, io::stdout, time::Duration};

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use git2::Repository;
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    console::{error_handling::miette_install, style},
    external::Git2,
    relates::{set_relates_to, RelateTo},
};

use crate::cli::app::Args;
use crate::errors::GitRelatesTo;

mod cli;
mod errors;

fn main() -> Result<()> {
    miette_install();

    let cli_args = Args::parse();

    // Simply print and exit if completion option is given.
    if let Some(completion) = cli_args.completion {
        let mut cmd = Args::command();
        let name = cmd.get_name().to_string();
        generate(completion, &mut cmd, name, &mut stdout());

        std::process::exit(0);
    }

    let relates_to = cli_args.issue_number.map(RelateTo::from).map_or_else(
        || Err::<RelateTo<'_>, GitRelatesTo>(GitRelatesTo::NoRelatesToMessageSet),
        Ok,
    )?;

    if repo_present() && !is_hook_present() {
        not_setup_warning();
    }

    let current_dir = env::current_dir().into_diagnostic()?;
    let mut vcs = Git2::try_from(current_dir)?;
    set_relates_to(
        &mut vcs,
        &relates_to,
        Duration::from_secs(cli_args.timeout * 60),
    )?;

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
        .filter(|path_buf| {
            path_buf
                .canonicalize()
                .ok()
                .is_some_and(|path| path.to_string_lossy().contains("mit-commit-msg"))
        })
        .is_some()
}

fn repo_present() -> bool {
    env::current_dir()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .is_some()
}
