//! The git mit binary

#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

use std::{convert::TryFrom, env, io::stdout, time::Duration};

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use git2::Repository;
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    console::{error_handling::miette_install, style},
    external::Git2,
    mit::{get_authors, set_commit_authors, Authors},
};

use crate::{cli::app::CliArgs, errors::UnknownAuthor};
mod cli;
mod errors;

fn main() -> Result<()> {
    miette_install();
    let cli_args = CliArgs::parse();

    // Simply print and exit if completion option is given.
    if let Some(completion) = cli_args.completion {
        let mut cmd = CliArgs::command();
        let name = cmd.get_name().to_string();
        generate(completion, &mut cmd, name, &mut stdout());

        std::process::exit(0);
    }

    let mut git_config = Git2::try_from(env::current_dir().into_diagnostic()?)?;
    let file_authors = get_authors(&cli_args)?;
    let authors = file_authors.merge(&Authors::try_from(&git_config)?);

    if repo_present() && !is_hook_present() {
        not_setup_warning();
    };

    let initials: Vec<&str> = cli_args.initials.iter().map(String::as_str).collect();
    let missing = authors.missing_initials(initials.clone());

    if !missing.is_empty() {
        return Err(UnknownAuthor {
            command: env::args().collect::<Vec<_>>().join(" "),
            missing_initials: missing.clone().into_iter().map(String::from).collect(),
        }
        .into());
    }

    set_commit_authors(
        &mut git_config,
        &authors.get(&initials),
        Duration::from_secs(cli_args.timeout * 60),
    )?;

    Ok(())
}

fn not_setup_warning() {
    style::warning("Hooks not found in this repository, your commits won't contain trailers, and lints will not be checked", Some("`git mit-install` will fix this"));
}

fn is_hook_present() -> bool {
    env::current_dir()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .map(|repo| repo.path().join("hooks").join("commit-msg"))
        .filter(|path_buf| {
            path_buf.canonicalize().ok().map_or(false, |path| {
                path.to_string_lossy().contains("mit-commit-msg")
            })
        })
        .is_some()
}

fn repo_present() -> bool {
    env::current_dir()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .is_some()
}
