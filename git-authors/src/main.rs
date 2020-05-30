use std::{
    env,
    error::Error,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use clap::{crate_authors, crate_version, App, Arg, ArgMatches};

use xdg::BaseDirectories;

use pb_commit_message_lints::{
    author::{entities::Authors, vcs::set_authors},
    external::vcs::Git2,
};

use crate::ExitCode::InitialNotMatchedToAuthor;
use pb_commit_message_lints::author::entities::Author;
use std::convert::TryFrom;

#[repr(i32)]
enum ExitCode {
    InitialNotMatchedToAuthor = 3,
}

const AUTHOR_INITIAL: &str = "initials";
const AUTHOR_FILE_PATH: &str = "file";
const AUTHOR_FILE_COMMAND: &str = "command";
const TIMEOUT: &str = "timeout";

fn main() {
    let config_path: String = config_file_path(env!("CARGO_PKG_NAME")).unwrap();
    let app = app(&config_path);
    let matches = app.get_matches();

    let users_config = get_users_config(&matches).unwrap();
    let authors_initials = get_author_initials(&matches).unwrap();
    let all_authors = Authors::try_from(users_config.as_str()).unwrap();
    let selected_authors = all_authors.get(&authors_initials);
    let initials_without_authors = find_initials_missing(authors_initials, &selected_authors);

    if !initials_without_authors.is_empty() {
        exit_initial_not_matched_to_author(&initials_without_authors);
    }

    let current_dir = env::current_dir().unwrap();
    let mut git_config = Git2::try_from(current_dir).unwrap();

    let authors = selected_authors.into_iter().flatten().collect::<Vec<_>>();
    set_authors(
        &mut git_config,
        &authors,
        Duration::from_secs(get_timeout(&matches).unwrap() * 60),
    )
    .unwrap()
}

fn exit_initial_not_matched_to_author(initials_without_authors: &[&str]) {
    eprintln!(
        r#"
Could not find the initials {}.

You can fix this by checking the initials are in the configuration file.
"#,
        initials_without_authors.join(", "),
    );

    std::process::exit(InitialNotMatchedToAuthor as i32);
}

fn find_initials_missing<'a>(
    authors_initials: Vec<&'a str>,
    selected_authors: &[Option<&Author>],
) -> Vec<&'a str> {
    selected_authors
        .iter()
        .zip(authors_initials)
        .filter_map(|(result, initial)| match result {
            None => Some(initial),
            Some(_) => None,
        })
        .collect()
}

fn app(config_file_path: &str) -> App {
    let cargo_package_name = String::from(env!("CARGO_PKG_NAME"));
    App::new(cargo_package_name)
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
                .default_value(config_file_path),
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
}

fn get_author_initials<'a>(matches: &'a ArgMatches) -> Option<Vec<&'a str>> {
    matches.values_of(AUTHOR_INITIAL).map(Iterator::collect)
}

fn get_users_config(matches: &ArgMatches) -> Result<String, Box<dyn Error>> {
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
        .ok_or_else(|| "No timeout set".into())
        .and_then(|x| x.parse().map_err(Box::from))
}

fn config_file_path(cargo_package_name: &str) -> Result<String, Box<dyn Error>> {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(Box::from)
        .and_then(|x| authors_config_file(&x))
        .map(|x| x.to_string_lossy().into())
}

fn authors_config_file(config_directory: &BaseDirectories) -> Result<PathBuf, Box<dyn Error>> {
    config_directory
        .place_config_file("authors.yml")
        .map_err(|x| x.into())
}
