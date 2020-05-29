extern crate pb_commit_message_lints;

use std::{env, fs};

use clap::{crate_authors, crate_version, App, Arg};
use git2::{Config, Repository};

use pb_commit_message_lints::{
    external::vcs::Git2,
    lints::{get_lint_configuration, lint, CommitMessage, LintCode, LintProblem},
};

const COMMIT_FILE_PATH_NAME: &str = "commit-file-path";

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
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
        .get_matches();

    let commit_file_path = matches.value_of(COMMIT_FILE_PATH_NAME).unwrap();
    let commit_message =
        fs::read_to_string(commit_file_path).expect("Something went wrong reading the file");

    let current_dir = env::current_dir().expect("Unable to retrieve current directory");

    let get_repository_config = |x: Repository| x.config();
    let get_default_config = |_| Config::open_default();
    let git_config = Repository::discover(current_dir)
        .and_then(get_repository_config)
        .or_else(get_default_config)
        .map(Git2::new)
        .expect("Couldn't load any git config");

    let (_, output) = create_output(
        commit_message.clone(),
        lint(
            &CommitMessage::new(&commit_message),
            get_lint_configuration(&git_config),
        ),
    );

    if let Some((message, exit_code)) = output {
        exit_error(&message, exit_code)
    }
}

fn create_output(
    commit_message: String,
    lint_problems: Vec<LintProblem>,
) -> (String, Option<(String, LintCode)>) {
    lint_problems.into_iter().fold(
        (commit_message, None),
        |(commit_message, output): (
            std::string::String,
            std::option::Option<(std::string::String, LintCode)>,
        ),
         item: LintProblem|
         -> (String, Option<(String, LintCode)>) {
            (
                commit_message.clone(),
                match output {
                    Some((existing_output, _)) => Some((
                        vec![existing_output, item.to_string()].join("\n\n"),
                        item.code(),
                    )),
                    None => Some((
                        vec![commit_message, item.to_string()].join("\n\n---\n\n"),
                        item.code(),
                    )),
                },
            )
        },
    )
}

fn exit_error(commit_message: &str, exit_code: LintCode) {
    eprintln!("{}", commit_message);

    std::process::exit(exit_code as i32);
}
