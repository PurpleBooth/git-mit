extern crate mit_commit_message_lints;

use std::env;

use mit_commit_message_lints::{
    external::vcs::Git2,
    lints::{lib::CommitMessage, lib::Lints, lint, Code, Problem},
};

use crate::cli::app;
use crate::errors::MitCommitMsgError;
use std::{convert::TryFrom, path::PathBuf};

fn main() -> Result<(), MitCommitMsgError> {
    let matches = app().get_matches();

    let commit_file_path = matches
        .value_of("commit-file-path")
        .map(PathBuf::from)
        .ok_or_else(|| errors::MitCommitMsgError::CommitPathMissing)?;

    let commit_message = CommitMessage::try_from(commit_file_path)?;

    let current_dir =
        env::current_dir().map_err(|err| MitCommitMsgError::new_io("$PWD".into(), &err))?;

    let mut git_config = Git2::try_from(current_dir)?;
    let lint_config = Lints::try_from_vcs(&mut git_config)?;
    let output = format_lint_problems(&commit_message, lint(&commit_message, lint_config));

    if let Some((message, exit_code)) = output {
        display_lint_err_and_exit(&message, exit_code)
    }

    Ok(())
}

mod cli;

fn format_lint_problems(
    original_message: &CommitMessage,
    lint_problems: Vec<Problem>,
) -> Option<(String, Code)> {
    let (_, message_and_code) = lint_problems.into_iter().fold(
        (original_message, None),
        |(commit_message, output), item| {
            (
                commit_message,
                match output {
                    Some((existing_output, _)) => Some((
                        vec![existing_output, item.to_string()].join("\n\n"),
                        item.code(),
                    )),
                    None => Some((
                        vec![commit_message.to_string(), item.to_string()].join("\n\n---\n\n"),
                        item.code(),
                    )),
                },
            )
        },
    );
    message_and_code
}

fn display_lint_err_and_exit(commit_message: &str, exit_code: Code) {
    eprintln!("{}", commit_message);

    std::process::exit(exit_code as i32);
}

mod errors;
