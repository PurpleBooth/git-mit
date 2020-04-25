extern crate pb_commit_message_lints;
use clap::{crate_authors, crate_version, App, Arg};
use pb_commit_message_lints::has_duplicated_trailers;
use std::env;
use std::fs;

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 2] = ["Signed-off-by", "Co-authored-by"];

fn main() -> std::io::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("commit-file-path")
                .help("Path to a temporary file that contains the commit message written by the developer")
                .index(1)
                .required(true)
        )
        .get_matches();

    let commit_file_path = matches.value_of("commit-file-path").unwrap();
    let commit_message =
        fs::read_to_string(commit_file_path).expect("Something went wrong reading the file");

    for trailer in &TRAILERS_TO_CHECK_FOR_DUPLICATES {
        if has_duplicated_trailers(&commit_message, trailer) {
            eprintln!(
                r#"
{}

Your commit cannot have the same name duplicated in the \"{}\" field
"#,
                commit_message, trailer
            );

            std::process::exit(1);
        }
    }

    Ok(())
}
