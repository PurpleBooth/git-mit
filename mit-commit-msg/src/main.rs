extern crate mit_commit_message_lints;

use std::{convert::TryFrom, env, path::PathBuf};

use arboard::Clipboard;
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use miette::{GraphicalTheme, IntoDiagnostic, Result};
use mit_commit::CommitMessage;
use mit_commit_message_lints::{
    console::style::print_completions,
    external,
    lints::read_from_toml_or_else_vcs,
};
use mit_lint::async_lint;

use crate::{cli::app, errors::AggregateProblem};

#[tokio::main]
async fn main() -> Result<()> {
    miette::set_panic_hook();
    if env::var("DEBUG_PRETTY_ERRORS").is_ok() {
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .force_graphical(true)
                    .terminal_links(false)
                    .graphical_theme(GraphicalTheme::unicode_nocolor())
                    .build(),
            )
        }))
        .unwrap();
    }

    let mut app = app();
    let matches = app.clone().get_matches();

    // Simply print and exit if completion option is given.
    if let Some(completion) = matches.value_of("completion") {
        match completion {
            "bash" => print_completions::<Bash>(&mut app),
            "elvish" => print_completions::<Elvish>(&mut app),
            "fish" => print_completions::<Fish>(&mut app),
            "powershell" => print_completions::<PowerShell>(&mut app),
            "zsh" => print_completions::<Zsh>(&mut app),
            _ => println!("Unknown completion"), // Never reached
        }

        std::process::exit(0);
    }

    let commit_file_path = match matches.value_of("commit-file-path") {
        None => Err(errors::MitCommitMsgError::CommitPathMissing),
        Some(path) => Ok(path),
    }
    .map(PathBuf::from)?;
    let commit_message = CommitMessage::try_from(commit_file_path)?;
    let current_dir = env::current_dir().into_diagnostic()?;

    let toml = external::read_toml(current_dir.clone())?;
    let mut git_config = external::Git2::try_from(current_dir)?;
    let lint_config = read_from_toml_or_else_vcs(&toml, &mut git_config)?;

    let lint_problems = async_lint(&commit_message, lint_config).await;
    if !lint_problems.is_empty() {
        if !matches
            .value_of_t::<bool>("copy-message-to-clipboard")
            .unwrap()
        {
        } else if let Ok(mut clipboard) = Clipboard::new() {
            let body = commit_message.get_body().to_string().trim().to_string();
            let trimmed_commit = if body.is_empty() {
                format!("{}", commit_message.get_subject())
            } else {
                format!(
                    "{}\n{}",
                    commit_message.get_subject(),
                    commit_message.get_body()
                )
            };

            clipboard.set_text(trimmed_commit).into_diagnostic()?;
        } else {
        };

        return AggregateProblem::to(lint_problems);
    }

    Ok(())
}

mod cli;

mod errors;
