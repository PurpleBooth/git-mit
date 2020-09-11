use std::env::current_dir;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::{env, fs};

use clap::ArgMatches;
use xdg::BaseDirectories;

use mit_commit_message_lints::mit::{get_config_authors, Authors};

use crate::errors::GitMitConfigError;
use crate::get_vcs;

use std::convert::{TryFrom, TryInto};

const PROBABLY_SAFE_FALLBACK_SHELL: &str = "/bin/sh";

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<(), GitMitConfigError>> {
    matches
        .subcommand_matches("mit")
        .filter(|subcommand| subcommand.subcommand_matches("generate").is_some())
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<(), GitMitConfigError> {
    let subcommand = matches
        .subcommand_matches("mit")
        .and_then(|matches| matches.subcommand_matches("generate"))
        .unwrap();

    let users_config = get_users_config(&subcommand)?;
    let all_authors = Authors::try_from(users_config.as_str());

    if let Err(error) = &all_authors {
        mit_commit_message_lints::console::exit_unparsable_author(error)
    }

    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir()?;
    let vcs = get_vcs(is_local, &current_dir)?;

    let vcs_authors = get_config_authors(&vcs)?;

    let toml: String = all_authors?.merge(&vcs_authors).try_into()?;

    mit_commit_message_lints::console::style::to_be_piped(&toml);
    Ok(())
}

fn get_users_config(matches: &ArgMatches) -> Result<String, GitMitConfigError> {
    match matches.value_of("command") {
        Some(command) => get_author_config_from_exec(command),
        None => get_author_config_from_file(matches),
    }
}

fn get_author_config_from_exec(command: &str) -> Result<String, GitMitConfigError> {
    let shell = env::var("SHELL").unwrap_or_else(|_| PROBABLY_SAFE_FALLBACK_SHELL.into());
    let output = Command::new(shell)
        .stderr(Stdio::inherit())
        .arg("-c")
        .arg(command)
        .output()?;
    Ok(String::from_utf8(output.stdout)?)
}

fn get_author_config_from_file(matches: &ArgMatches) -> Result<String, GitMitConfigError> {
    get_author_file_path(&matches)
        .ok_or(GitMitConfigError::AuthorFileNotSet)
        .and_then(|path| match path {
            "$HOME/.config/git-mit/mit.yml" => config_path(env!("CARGO_PKG_NAME")),
            _ => Ok(path.into()),
        })
        .and_then(|path| Ok(fs::read_to_string(&path)?))
}

fn get_author_file_path(matches: &ArgMatches) -> Option<&str> {
    matches.value_of("file")
}

fn config_path(cargo_package_name: &str) -> Result<String, GitMitConfigError> {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(GitMitConfigError::from)
        .and_then(|base| authors_config_file(&base))
        .map(|path| path.to_string_lossy().into())
}

fn authors_config_file(config_directory: &BaseDirectories) -> Result<PathBuf, GitMitConfigError> {
    Ok(config_directory.place_config_file("mit.toml")?)
}
