use std::{env, fs, fs::File, io::Write};

use clap::{crate_authors, crate_version, App, Arg};
use git2::{Config, Repository};
use itertools::Itertools;

use pb_commit_message_lints::{get_coauthor_configuration, Author, Git2VcsConfig};

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

    let commit_message_path = matches.value_of("commit-message-path").unwrap();

    let current_dir = env::current_dir().expect("Unable to retrieve current directory");

    let get_repository_config = |x: Repository| x.config();
    let get_default_config = |_| Config::open_default();
    let snapshot_config = |mut x: git2::Config| x.snapshot();

    let git_config = Repository::discover(current_dir)
        .and_then(get_repository_config)
        .or_else(get_default_config)
        .and_then(snapshot_config)
        .map(Git2VcsConfig::new)
        .expect("Could not freeze git config");

    if let Some(authors) = get_coauthor_configuration(&git_config) {
        let commit_message =
            fs::read_to_string(commit_message_path).expect("Could not read commit message");
        let write_co_author_trailer =
            |x: &Author| format!("Co-authored-by: {} <{}>", x.name(), x.email());
        File::create(commit_message_path)
            .expect("Unable to open commit message file")
            .write_all(
                format!(
                    r#"{}
{}
"#,
                    commit_message,
                    authors.iter().map(write_co_author_trailer).join("\n")
                )
                .as_bytes(),
            )
            .expect("Failed to write an updated commit message");
    }
}
