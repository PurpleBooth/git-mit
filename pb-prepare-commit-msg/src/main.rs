use std::{env, fs};

use clap::{crate_authors, crate_version, App, Arg};
use git2::{Config, Repository};
use pb_commit_author::get_author_configuration;
use std::{fs::File, io::Write};

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

    let git_config = Repository::discover(current_dir)
        .and_then(|x| x.config())
        .or_else(|_| Config::open_default())
        .expect("Couldn't load any git config");

    if let Some(()) = get_author_configuration(&git_config) {
        let commit_message =
            fs::read_to_string(commit_message_path).expect("Could not read commit message");
        File::create(commit_message_path)
            .expect("Unable to open commit message file")
            .write_all(
                format!(
                    r#"{}
Co-authored-by: Annie Example <test@example.com>
"#,
                    commit_message
                )
                .as_bytes(),
            )
            .expect("Failed to write an updated commit message");
    }
}
