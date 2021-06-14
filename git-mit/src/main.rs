use std::{
    convert::TryFrom,
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use clap::ArgMatches;

use mit_commit_message_lints::console::exit_initial_not_matched_to_author;
use mit_commit_message_lints::console::exit_unparsable_author;
use mit_commit_message_lints::mit::get_config_authors;
use mit_commit_message_lints::{
    external::Git2,
    mit::{set_commit_authors, Author, Authors},
};

use crate::errors::GitMitError;
use crate::errors::GitMitError::{NoAuthorInitialsProvided, NoTimeoutSet};

fn main() -> Result<(), errors::GitMitError> {
    let matches = cli::app().get_matches();
    let users_config = get_users_config(&matches)?;
    let authors_initials = get_author_initials(&matches).ok_or(NoAuthorInitialsProvided)?;

    let current_dir =
        env::current_dir().map_err(|error| GitMitError::new_io("$PWD".into(), &error))?;

    let mut git_config = Git2::try_from(current_dir)?;
    let config_authors = Authors::try_from(users_config.as_str());

    if let Err(error) = &config_authors {
        exit_unparsable_author(error);
    }

    let all_authors = config_authors?.merge(&get_config_authors(&git_config)?);

    let selected_authors = all_authors.get(&authors_initials);
    let initials_without_authors = find_initials_missing(authors_initials, &selected_authors);

    if !initials_without_authors.is_empty() {
        exit_initial_not_matched_to_author(&initials_without_authors);
    }

    let authors = selected_authors.into_iter().flatten().collect::<Vec<_>>();
    set_commit_authors(
        &mut git_config,
        &authors,
        Duration::from_secs(get_timeout(&matches)? * 60),
    )?;

    Ok(())
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

mod cli;

fn get_author_initials(matches: &ArgMatches) -> Option<Vec<&str>> {
    matches.values_of("initials").map(Iterator::collect)
}

fn get_users_config(matches: &ArgMatches) -> Result<String, GitMitError> {
    match matches.value_of("command") {
        Some(command) => get_author_config_from_exec(command),
        None => get_author_config_from_file(matches),
    }
}

fn get_author_config_from_exec(command: &str) -> Result<String, GitMitError> {
    let commandline = shell_words::split(command)?;
    Command::new(commandline.first().unwrap_or(&String::from("")))
        .stderr(Stdio::inherit())
        .args(commandline.iter().skip(1).collect::<Vec<_>>())
        .output()
        .map_err(|error| GitMitError::new_exec(command.into(), &error))
        .and_then(|x| String::from_utf8(x.stdout).map_err(GitMitError::from))
}

fn get_author_config_from_file(matches: &ArgMatches) -> Result<String, GitMitError> {
    get_author_file_path(matches)
        .ok_or(GitMitError::AuthorFileNotSet)
        .and_then(|path| match path {
            "$HOME/.config/git-mit/mit.yml" => config_path(env!("CARGO_PKG_NAME")),
            _ => Ok(path.into()),
        })
        .and_then(|path| {
            fs::read_to_string(&path).map_err(|error| GitMitError::new_io(path, &error))
        })
}

fn get_author_file_path(matches: &ArgMatches) -> Option<&str> {
    matches.value_of("file")
}

fn get_timeout(matches: &ArgMatches) -> Result<u64, GitMitError> {
    matches
        .value_of("timeout")
        .ok_or(NoTimeoutSet)
        .and_then(|x| x.parse().map_err(GitMitError::from))
}

#[cfg(not(target_os = "windows"))]
fn config_path(cargo_package_name: &str) -> Result<String, GitMitError> {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(GitMitError::from)
        .and_then(|base| authors_config_file(&base))
        .map(|path| path.to_string_lossy().into())
}

#[cfg(target_os = "windows")]
fn config_path(cargo_package_name: &str) -> Result<String, GitMitError> {
    std::env::var("APPDATA")
        .map(|x| {
            PathBuf::from(x)
                .join(cargo_package_name)
                .join("mit.toml")
                .to_string_lossy()
                .into()
        })
        .map_err(|error| GitMitError::AppDataMissing(error))
}

#[cfg(not(target_os = "windows"))]
fn authors_config_file(config_directory: &xdg::BaseDirectories) -> Result<PathBuf, GitMitError> {
    config_directory
        .place_config_file("mit.toml")
        .map_err(|error| GitMitError::new_io("<config_dir>/mit.toml".into(), &error))
}

mod errors;
