extern crate pb_commit_message_lints;

use std::env;

use clap::{crate_authors, crate_version, App, Arg};

use pb_commit_message_lints::{
    external::vcs::Git2,
    lints::{get_lint_configuration, lint, CommitMessage, LintCode, LintProblem},
};
use std::{convert::TryFrom, path::PathBuf};

const COMMIT_FILE_PATH_NAME: &str = "commit-file-path";

fn main() {
    let matches = app().get_matches();

    let commit_file_path = matches
        .value_of(COMMIT_FILE_PATH_NAME)
        .map(PathBuf::from)
        .unwrap();
    let commit_message = CommitMessage::try_from(commit_file_path).unwrap();

    let current_dir = env::current_dir().unwrap();
    let git_config = Git2::try_from(current_dir).unwrap();

    let output = format_lint_problems(
        &commit_message,
        lint(
            &commit_message,
            get_lint_configuration(&git_config).unwrap(),
        ),
    );

    if let Some((message, exit_code)) = output {
        exit_error(&message, exit_code)
    }
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

fn exit_error(commit_message: &str, exit_code: LintCode) {
    eprintln!("{}", commit_message);

    std::process::exit(exit_code as i32);
}
