extern crate mit_commit_message_lints;

use std::env;
use std::{convert::TryFrom, path::PathBuf};

use mit_commit::CommitMessage;

use mit_commit_message_lints::{
    external,
    lints::{lint, Lints},
};

use crate::cli::app;
use crate::errors::MitCommitMsgError;
use copypasta::{ClipboardContext, ClipboardProvider};
use mit_commit_message_lints::console::exit_lint_problem;
use std::str::FromStr;

fn main() -> Result<(), MitCommitMsgError> {
    let matches = app().get_matches();

    let commit_file_path = matches
        .value_of("commit-file-path")
        .map(PathBuf::from)
        .ok_or(errors::MitCommitMsgError::CommitPathMissing)?;

    let commit_message = CommitMessage::try_from(commit_file_path)?;

    let current_dir =
        env::current_dir().map_err(|err| MitCommitMsgError::new_io("$PWD".into(), &err))?;

    let toml = external::read_toml(current_dir.clone())?;
    let mut git_config = external::Git2::try_from(current_dir)?;
    let lint_config = Lints::get_from_toml_or_else_vcs(&toml, &mut git_config)?;

    let lint_problems = lint(&commit_message, lint_config);
    if !lint_problems.is_empty() {
        let clipboard_used =
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

                clipboard.set_contents(trimmed_commit)?;
                true
            } else {
                false
            };

        exit_lint_problem(&commit_message, lint_problems, clipboard_used)
    }

    Ok(())
}

mod cli;

mod errors;
