use std::{
    env,
    error::Error,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use git2::{Config, Repository};
use xdg::BaseDirectories;

use pb_commit_message_lints::{
    author::{
        entities::{Author, Authors},
        vcs::set_authors,
        yaml::get_authors_from_user_config,
    },
    external::vcs::Git2,
};

use crate::ExitCode::InitialNotMatchedToAuthor;

#[repr(i32)]
enum ExitCode {
    InitialNotMatchedToAuthor = 3,
}

const AUTHOR_INITIAL: &str = "initials";
const AUTHOR_FILE_PATH: &str = "file";
const AUTHOR_FILE_COMMAND: &str = "command";
const TIMEOUT: &str = "timeout";

fn main() {
    let cargo_package_name = env!("CARGO_PKG_NAME");
    let default_config_file = config_file_path(cargo_package_name).unwrap();

    let matches = App::new(cargo_package_name)
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(AUTHOR_INITIAL)
                .help("Initials of the authors to put in the commit")
                .multiple(true)
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name(AUTHOR_FILE_PATH)
                .short("c")
                .long("config")
                .help("Path to a file where authors initials, emails and names can be found")
                .env("GIT_AUTHORS_CONFIG")
                .default_value(&default_config_file),
        )
        .arg(
            Arg::with_name(AUTHOR_FILE_COMMAND)
                .short("e")
                .long("exec")
                .help(
                    "Execute a command to generate the author configuration, stdout will be \
                     captured and used instead of the file, if both this and the file is present, \
                     this takes precedence",
                )
                .env("GIT_AUTHORS_EXEC"),
        )
        .arg(
            Arg::with_name(TIMEOUT)
                .short("t")
                .long("timeout")
                .help("Number of minutes to expire the configuration in")
                .env("GIT_AUTHORS_TIMEOUT")
                .default_value("60"),
        )
        .get_matches();

    let author_config = get_author_config(&matches).unwrap();
    let authors_initials = get_author_initials(&matches).unwrap();
    let yaml_authors: Authors = get_authors_from_user_config(&author_config).unwrap();
    let selected_authors: Vec<Option<&Author>> = yaml_authors.get(&authors_initials);
    let failed_authors: Vec<_> = selected_authors
        .iter()
        .zip(authors_initials)
        .filter_map(|(result, initial)| match result {
            None => Some(initial),
            Some(_) => None,
        })
        .collect();

    if !failed_authors.is_empty() {
        eprintln!(
            r#"
Could not find the initials {}.

You can fix this by checking the initials are in the configuration file.
"#,
            failed_authors.join(", "),
        );

        std::process::exit(InitialNotMatchedToAuthor as i32);
    }

    let current_dir = env::current_dir().unwrap();
    let mut git_config = Repository::discover(current_dir)
        .and_then(|x: Repository| x.config())
        .or_else(|_| Config::open_default())
        .map(Git2::new)
        .unwrap();

    let authors = selected_authors.into_iter().flatten().collect::<Vec<_>>();
    set_authors(
        &mut git_config,
        &authors,
        Duration::from_secs(get_timeout(&matches).unwrap() * 60),
    )
    .unwrap()
}

fn get_author_initials<'a>(matches: &'a ArgMatches) -> Option<Vec<&'a str>> {
    matches.values_of(AUTHOR_INITIAL).map(Iterator::collect)
}

fn get_author_config(matches: &ArgMatches) -> Result<String, Box<dyn Error>> {
    match matches.value_of(AUTHOR_FILE_COMMAND) {
        Some(command) => get_author_config_from_exec(command),
        None => get_author_config_from_file(matches),
    }
}

fn get_author_config_from_exec(command: &str) -> Result<String, Box<dyn Error>> {
    Command::new(env::var("SHELL").unwrap_or_else(|_| String::from("sh")))
        .stderr(Stdio::inherit())
        .arg("-c")
        .arg(command)
        .output()
        .map_err(Box::<dyn Error>::from)
        .and_then(|x| String::from_utf8(x.stdout).map_err(Box::from))
}

fn get_author_config_from_file(matches: &ArgMatches) -> Result<String, Box<dyn Error>> {
    get_author_file_path(&matches)
        .ok_or_else(|| -> Box<dyn Error> { "Expected a author file path, didn't find one".into() })
        .map_err(Box::from)
        .and_then(|x| fs::read_to_string(x).map_err(Box::from))
}

fn get_author_file_path<'a>(matches: &'a ArgMatches) -> Option<&'a str> {
    matches.value_of(AUTHOR_FILE_PATH)
}

fn get_timeout(matches: &ArgMatches) -> Result<u64, Box<dyn Error>> {
    matches
        .value_of(TIMEOUT)
        .ok_or_else(|| -> Box<dyn Error> { "No timeout set".into() })
        .and_then(|x| x.parse().map_err(Box::from))
}

fn config_file_path(cargo_package_name: &str) -> Result<String, Box<dyn Error>> {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(Box::<dyn std::error::Error>::from)
        .and_then(|x| authors_config_file(&x))
        .map(|x| x.to_string_lossy().into())
}

fn authors_config_file(config_directory: &BaseDirectories) -> Result<PathBuf, Box<dyn Error>> {
    config_directory
        .place_config_file("authors.yml")
        .map_err(|x| x.into())
}
