//! The pre-commit binary

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

use std::{convert::TryFrom, env, io::stdout};

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    console::error_handling::miette_install,
    external::Git2,
    mit::{get_commit_coauthor_configuration, AuthorState},
};

use crate::{
    cli::Args,
    errors::{NoAuthorError, StaleAuthorError},
};

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

    let current_dir = env::current_dir().into_diagnostic()?;
    let git_config = Git2::try_from(current_dir)?;
    let co_author_configuration = get_commit_coauthor_configuration(&git_config)?;

    if let AuthorState::Timeout(time) = co_author_configuration {
        return Err(StaleAuthorError::new(time).into());
    }

    if co_author_configuration.is_none() {
        return Err(NoAuthorError {}.into());
    }

    Ok(())
}

mod cli;
mod errors;
