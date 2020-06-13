extern crate pb_commit_message_lints;

use std::env;

use clap::{crate_authors, crate_version, App, Arg};

use crate::PbCommitMessageError::PbCommitMessageLints;
use pb_commit_message_lints::{
    errors::PbCommitMessageLintsError,
    external::vcs::Git2,
    lints::{lib::CommitMessage, lib::Lints, lint, LintCode, LintProblem},
};

use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter},
    path::PathBuf,
};

const COMMIT_FILE_PATH_NAME: &str = "commit-file-path";

fn main() -> Result<(), PbCommitMessageError> {
    let matches = app().get_matches();

    let commit_file_path = matches
        .value_of(COMMIT_FILE_PATH_NAME)
        .map(PathBuf::from)
        .ok_or_else(|| PbCommitMessageError::CommitPathMissing)?;

    let commit_message = CommitMessage::try_from(commit_file_path)?;

    let current_dir =
        env::current_dir().map_err(|err| PbCommitMessageError::new_io("$PWD".into(), &err))?;

    let mut git_config = Git2::try_from(current_dir)?;

    let lint_config = Lints::try_from_vcs(&mut git_config)?;
    let output = format_lint_problems(&commit_message, lint(&commit_message, lint_config));

    if let Some((message, exit_code)) = output {
        display_lint_err_and_exit(&message, exit_code)
    }

    Ok(())
}

fn app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(COMMIT_FILE_PATH_NAME)
                .help(
                    "Path to a temporary file that contains the commit message written by the \
                     developer",
                )
                .index(1)
                .required(true),
        )
}

fn format_lint_problems(
    original_message: &CommitMessage,
    lint_problems: Vec<LintProblem>,
) -> Option<(String, LintCode)> {
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

fn display_lint_err_and_exit(commit_message: &str, exit_code: LintCode) {
    eprintln!("{}", commit_message);

    std::process::exit(exit_code as i32);
}

#[derive(Debug)]
enum PbCommitMessageError {
    CommitPathMissing,
    PbCommitMessageLints(PbCommitMessageLintsError),
    Io(String, String),
}

impl PbCommitMessageError {
    fn new_io(location: String, error: &std::io::Error) -> PbCommitMessageError {
        PbCommitMessageError::Io(location, format!("{}", error))
    }
}

impl Display for PbCommitMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PbCommitMessageLints(error) => write!(f, "{}", error),
            PbCommitMessageError::Io(file_source, error) => write!(
                f,
                "Failed to read git config from `{}`:\n{}",
                file_source, error
            ),
            PbCommitMessageError::CommitPathMissing => write!(f, "Expected file path name",),
        }
    }
}

impl From<PbCommitMessageLintsError> for PbCommitMessageError {
    fn from(err: PbCommitMessageLintsError) -> Self {
        PbCommitMessageLints(err)
    }
}

impl Error for PbCommitMessageError {}
