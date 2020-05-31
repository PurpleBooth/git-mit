use std::{env, fs::File, io::Write, process};

use clap::{crate_authors, crate_version, App, Arg};

use itertools::Itertools;

use pb_commit_message_lints::{
    author::{entities::Author, vcs::get_coauthor_configuration},
    external::vcs::Git2,
    lints::CommitMessage,
};
use std::{convert::TryFrom, path::PathBuf};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("commit-message-path")
                .help("The name of the file that contains the commit log message")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("commit-message-source")
                .help(
                    "The commit message, and can be: message (if a -m or -F option was given to \
                     git); template (if a -t option was given or the configuration option \
                     commit.template is set in git); merge (if the commit is a merge or a \
                     .git/MERGE_MSG file exists); squash (if a .git/SQUASH_MSG file exists); or \
                     commit",
                )
                .index(2)
                .required(false),
        )
        .arg(
            Arg::with_name("commit-sha")
                .help("Commit SHA-1 (if a -c, -C or --amend option was given to git).")
                .index(3)
                .required(false),
        )
        .get_matches();

    let commit_message_path = matches
        .value_of("commit-message-path")
        .map(PathBuf::from)
        .expect("Expected commit file path");
    let current_dir = env::current_dir().unwrap_or_else(|err| {
        eprintln!("Failed to get current working directory:\n{}", err);
        process::exit(1);
    });

    let mut git_config = Git2::try_from(current_dir).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Some(authors) = get_coauthor_configuration(&mut git_config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    }) {
        append_coauthors_to_commit_message(commit_message_path, &authors).unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        })
    }
}

fn append_coauthors_to_commit_message(
    commit_message_path: PathBuf,
    authors: &[Author],
) -> Result<(), Box<dyn std::error::Error>> {
    let commit_message = CommitMessage::try_from(commit_message_path.clone())?;
    let mut file = File::create(commit_message_path)?;
    file.write_all(
        format!(
            r#"{}
{}
"#,
            authors
                .iter()
                .map(|x| format!("Co-authored-by: {} <{}>", x.name(), x.email()))
                .join("\n"),
            commit_message
        )
        .as_bytes(),
    )
    .map_err(Box::from)
}
