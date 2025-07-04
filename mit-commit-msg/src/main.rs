//! The binary for the commit-msg hook

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

use arboard::Clipboard;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use miette::{IntoDiagnostic, Result};
use mit_commit::CommitMessage;
use mit_commit_message_lints::{
    console::error_handling::miette_install, external, lints::read_from_toml_or_else_vcs,
};
use mit_lint::async_lint;

use crate::{cli::Args, errors::AggregateProblem};

#[tokio::main]
async fn main() -> Result<()> {
    miette_install();

    let cli_args = Args::parse();

    // Simply print and exit if completion option is given.
    if let Some(completion) = cli_args.completion {
        let mut cmd = Args::command();
        let name = cmd.get_name().to_string();
        generate(completion, &mut cmd, name, &mut stdout());

        std::process::exit(0);
    }

    let commit_file_path = cli_args
        .commit_file_path
        .ok_or(errors::MitCommitMsgError::CommitPathMissing)?;
    let commit_message = CommitMessage::try_from(commit_file_path).into_diagnostic()?;
    let current_dir = env::current_dir().into_diagnostic()?;

    let toml = external::read_toml(current_dir.clone())?;
    let git_config = external::Git2::try_from(current_dir)?;
    let lint_config = read_from_toml_or_else_vcs(&toml, &git_config)?;

    let lint_problems = async_lint(&commit_message, &lint_config).await;
    if lint_problems.is_empty() {
        return Ok(());
    }

    if let Ok(mut clipboard) = Clipboard::new() {
        let trimmed_commit = vec![
            commit_message.get_subject().to_string(),
            commit_message.get_body().to_string().trim().to_string(),
        ]
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
        clipboard.set_text(trimmed_commit).into_diagnostic()?;
    }

    AggregateProblem::to(lint_problems)
}

mod cli;
mod errors;
