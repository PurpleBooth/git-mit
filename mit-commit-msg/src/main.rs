extern crate mit_commit_message_lints;

use std::{convert::TryFrom, env, path::PathBuf, str::FromStr};

use copypasta::{ClipboardContext, ClipboardProvider};
use miette::{GraphicalTheme, IntoDiagnostic, Result};
use mit_commit::CommitMessage;
use mit_commit_message_lints::{external, lints::read_from_toml_or_else_vcs};
use mit_lint::async_lint;

use crate::{
    cli::app,
    errors::{AggregateProblem, MitCommitMsgError},
};

#[tokio::main]
async fn main() -> Result<()> {
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

    let matches = app().get_matches();

    let commit_file_path = match matches.value_of("commit-file-path") {
        None => Err(errors::MitCommitMsgError::CommitPathMissing),
        Some(path) => Ok(path),
    }
    .map(PathBuf::from)?;
    let commit_message = CommitMessage::try_from(commit_file_path).into_diagnostic()?;
    let current_dir = env::current_dir().into_diagnostic()?;

    let toml = external::read_toml(current_dir.clone())?;
    let mut git_config = external::Git2::try_from(current_dir)?;
    let lint_config = read_from_toml_or_else_vcs(&toml, &mut git_config)?;

    let lint_problems = async_lint(&commit_message, lint_config).await;
    if !lint_problems.is_empty() {
        let _clipboard_used =
            if !FromStr::from_str(matches.value_of("copy-message-to-clipboard").unwrap())
                .unwrap_or(true)
            {
                false
            } else if let Ok(mut clipboard) = ClipboardContext::new() {
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

                clipboard
                    .set_contents(trimmed_commit)
                    .map_err(|e| MitCommitMsgError::Clipboard(e))?;
                true
            } else {
                false
            };

        return AggregateProblem::to(lint_problems);
    }

    Ok(())
}

mod cli;

mod errors;
