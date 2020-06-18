use std::{
    convert::TryFrom,
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use clap::ArgMatches;
use xdg::BaseDirectories;

use mit_commit_message_lints::{
    author::{
        entities::{Author, Authors},
        vcs::set_authors,
    },
    external::vcs::Git2,
};

use crate::errors::GitMitError;
use crate::errors::GitMitError::{NoAuthorInitialsProvided, NoTimeoutSet};
use crate::ExitCode::InitialNotMatchedToAuthor;

#[repr(i32)]
enum ExitCode {
    InitialNotMatchedToAuthor = 3,
}

const PROBABLY_SAFE_FALLBACK_SHELL: &str = "/bin/sh";

fn main() -> Result<(), errors::GitMitError> {
    let path = config_path(env!("CARGO_PKG_NAME"))?;
    let matches = cli::app(&path).get_matches();
    let users_config = get_users_config(&matches)?;
    let authors_initials = get_author_initials(&matches).ok_or_else(|| NoAuthorInitialsProvided)?;

    let all_authors = Authors::try_from(users_config.as_str())?;
    let selected_authors = all_authors.get(&authors_initials);
    let initials_without_authors = find_initials_missing(authors_initials, &selected_authors);

    if !initials_without_authors.is_empty() {
        exit_initial_not_matched_to_author(&initials_without_authors);
    }

    let current_dir =
        env::current_dir().map_err(|error| GitMitError::new_io("$PWD".into(), &error))?;

    let mut git_config = Git2::try_from(current_dir)?;

    let authors = selected_authors.into_iter().flatten().collect::<Vec<_>>();
    set_authors(
        &mut git_config,
        &authors,
        Duration::from_secs(get_timeout(&matches)? * 60),
    )?;

    Ok(())
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
    let shell = env::var("SHELL").unwrap_or_else(|_| PROBABLY_SAFE_FALLBACK_SHELL.into());
    Command::new(shell)
        .stderr(Stdio::inherit())
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|error| GitMitError::new_exec(command.into(), &error))
        .and_then(|x| String::from_utf8(x.stdout).map_err(GitMitError::from))
}

fn get_author_config_from_file(matches: &ArgMatches) -> Result<String, GitMitError> {
    get_author_file_path(&matches)
        .ok_or_else(|| GitMitError::AuthorFileNotSet)
        .and_then(|path| {
            fs::read_to_string(path).map_err(|error| GitMitError::new_io(path.into(), &error))
        })
}

fn get_author_file_path(matches: &ArgMatches) -> Option<&str> {
    matches.value_of("file")
}

fn get_timeout(matches: &ArgMatches) -> Result<u64, GitMitError> {
    matches
        .value_of("timeout")
        .ok_or_else(|| NoTimeoutSet)
        .and_then(|x| x.parse().map_err(GitMitError::from))
}

fn config_path(cargo_package_name: &str) -> Result<String, GitMitError> {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(GitMitError::from)
        .and_then(|base| authors_config_file(&base))
        .map(|path| path.to_string_lossy().into())
}

fn authors_config_file(config_directory: &BaseDirectories) -> Result<PathBuf, GitMitError> {
    config_directory
        .place_config_file("mit.yml")
        .map_err(|error| GitMitError::new_io("<config_dir>/author.yml".into(), &error))
}

mod errors;
