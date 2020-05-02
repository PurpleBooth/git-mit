extern crate pb_commit_message_lints;

use std::{env, fs};

use clap::{crate_authors, crate_version, App, Arg};
use git2::{Config, Repository};

use pb_commit_message_lints::{
    get_lint_configuration,
    has_duplicated_trailers,
    has_missing_pivotal_tracker_id,
    Lints,
    Lints::DuplicatedTrailers,
};

const COMMIT_FILE_PATH_NAME: &str = "commit-file-path";
const FIELD_SINGULAR: &str = "field";
const FIELD_PLURAL: &str = "fields";

fn main() -> std::io::Result<()> {
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

    let git_config = Repository::discover(current_dir)
        .and_then(|x| x.config())
        .or_else(|_| Config::open_default())
        .expect("Couldn't load any git config");

    let checks =
        get_lint_configuration(&git_config).expect("Couldn't parse the configuration in git");

    for check in checks {
        match check {
            DuplicatedTrailers => {
                if let Some(trailers) = has_duplicated_trailers(&commit_message) {
                    let mut fields = FIELD_SINGULAR;
                    if trailers.len() > 1 {
                        fields = FIELD_PLURAL
                    }

                    eprintln!(
                        r#"
{}

Your commit cannot have the same name duplicated in the "{}" {}
"#,
                        commit_message,
                        trailers.join("\", \""),
                        fields
                    );

                    std::process::exit(1);
                }
            }
            Lints::PivotalTrackerIdMissing => {
                if let Some(()) = has_missing_pivotal_tracker_id(&commit_message) {
                    eprintln!(
                        r#"
{}

Your commit is missing a Pivotal Tracker Id

Examples:
[Delivers #12345678]
[fixes #12345678]
[finishes #12345678]
[#12345884 #12345678]
[#12345884,#12345678]
[#12345678],[#12345884]
This will address [#12345884]
"#,
                        commit_message
                    );

                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
