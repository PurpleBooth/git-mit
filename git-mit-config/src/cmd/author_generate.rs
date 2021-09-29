use std::{
    convert::TryFrom,
    env,
    env::current_dir,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::ArgMatches;
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{console::style::author_table, mit::Authors};

use crate::{errors::GitMitConfigError, get_vcs};

pub(crate) fn run_on_match(matches: &ArgMatches) -> Option<Result<()>> {
    matches
        .subcommand_matches("mit")
        .filter(|subcommand| {
            subcommand.subcommand_matches("generate").is_some()
                || subcommand.subcommand_matches("available").is_some()
        })
        .map(|_| run(matches))
}

fn run(matches: &ArgMatches) -> Result<()> {
    let subcommand = matches
        .subcommand_matches("mit")
        .and_then(|matches| {
            matches
                .subcommand_matches("generate")
                .or_else(|| matches.subcommand_matches("available"))
        })
        .unwrap();

    let users_config = get_users_config(subcommand)?;
    let config_authors = Authors::try_from(users_config.as_str())?;

    let is_local = Some("local") == matches.value_of("scope");
    let current_dir = current_dir().into_diagnostic()?;
    let vcs = get_vcs(is_local, &current_dir)?;
    let vcs_authors = Authors::try_from(&vcs)?;

    let authors = config_authors.merge(&vcs_authors);

    let output: String = if matches
        .subcommand_matches("mit")
        .and_then(|x| x.subcommand_matches("generate"))
        .is_some()
    {
        to_toml(authors)
    } else {
        author_table(&authors)
    };

    mit_commit_message_lints::console::style::to_be_piped(&output);
    Ok(())
}

fn to_toml(authors: Authors) -> String {
    String::from(authors).trim().to_string()
}

fn get_users_config(matches: &ArgMatches) -> Result<String> {
    match matches.value_of("command") {
        Some(command) => get_author_config_from_exec(command),
        None => get_author_config_from_file(matches),
    }
}

fn get_author_config_from_exec(command: &str) -> Result<String> {
    let commandline = shell_words::split(command).into_diagnostic()?;
    let output = Command::new(commandline.first().unwrap_or(&String::from("")))
        .stderr(Stdio::inherit())
        .args(commandline.iter().skip(1).collect::<Vec<_>>())
        .output()
        .into_diagnostic()?;
    String::from_utf8(output.stdout).into_diagnostic()
}

fn get_author_config_from_file(matches: &ArgMatches) -> Result<String> {
    get_author_file_path(matches)
        .ok_or(GitMitConfigError::AuthorFileNotSet)
        .into_diagnostic()
        .and_then(|path| match path {
            "$HOME/.config/git-mit/mit.toml" => config_path(env!("CARGO_PKG_NAME")),
            _ => Ok(path.into()),
        })
        .map(|path| fs::read_to_string(&path).unwrap_or_default())
}

fn get_author_file_path(matches: &ArgMatches) -> Option<&str> {
    matches.value_of("file")
}

#[cfg(not(target_os = "windows"))]
fn config_path(cargo_package_name: &str) -> Result<String> {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .into_diagnostic()
        .and_then(|base| authors_config_file(&base))
        .map(|path| path.to_string_lossy().into())
}

#[cfg(target_os = "windows")]
fn config_path(cargo_package_name: &str) -> Result<String> {
    std::env::var("APPDATA")
        .map(|x| {
            PathBuf::from(x)
                .join(cargo_package_name)
                .join("mit.toml")
                .to_string_lossy()
                .into()
        })
        .into_diagnostic()
}

#[cfg(not(target_os = "windows"))]
fn authors_config_file(config_directory: &xdg::BaseDirectories) -> Result<PathBuf> {
    config_directory
        .place_config_file("mit.toml")
        .into_diagnostic()
}
