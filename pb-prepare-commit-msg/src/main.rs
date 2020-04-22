use std::env;
use std::os::unix::process::CommandExt;
use std::process;

use clap::{crate_authors, crate_version, App, Arg};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("commit-message-path")
                .help("The name of the file that contains the commit log message")
                .index(1)
                .required(true)
        )
        .arg(
            Arg::with_name("commit-message-source")
                .help("The second is the source of the commit message, and can be: message (if a -m or -F option was given to git); template (if a -t option was given or the configuration option commit.template is set in git); merge (if the commit is a merge or a .git/MERGE_MSG file exists); squash (if a .git/SQUASH_MSG file exists); or commit")
                .index(2)
                .required(false)
        )
        .arg(
            Arg::with_name("commit-sha")
                .help("Commit SHA-1 (if a -c, -C or --amend option was given to git).")
                .index(3)
                .required(false)
        )
        .get_matches();

    let mut arguments: Vec<String> = vec![];

    if let Some(config) = matches.value_of("commit-message-path") {
        arguments.push(config.to_string())
    }

    if let Some(config) = matches.value_of("commit-message-source") {
        arguments.push(config.to_string())
    }

    if let Some(config) = matches.value_of("commit-sha") {
        arguments.push(config.to_string())
    }

    arguments.extend(env::args().skip(1).collect::<Vec<String>>().iter().cloned());

    let cmd = "git";
    let err = process::Command::new(cmd).args(arguments).exec();
    panic!("panic!: {}", err)
}
