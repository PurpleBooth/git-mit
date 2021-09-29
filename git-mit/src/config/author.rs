use std::{
    convert::TryFrom,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::mit::Authors;

use crate::{cli::args::Args, errors::GitMitError};

pub(crate) fn load(args: &Args) -> Result<Authors> {
    let toml = match args.command() {
        Some(command) => from_exec(command),
        None => from_file(args),
    }?;

    let authors: Authors = Authors::try_from(&*toml)?;
    Ok(authors)
}

fn from_file(args: &Args) -> Result<String> {
    match args.author_file() {
        None => Err(GitMitError::AuthorFileNotSet.into()),
        Some(path) => Ok(path),
    }
    .and_then(|path| match path {
        "$HOME/.config/git-mit/mit.toml" => author_file_path(env!("CARGO_PKG_NAME")),
        _ => Ok(path.into()),
    })
    .map(|path| fs::read_to_string(&path).unwrap_or_default())
}

#[cfg(not(target_os = "windows"))]
fn author_file_path(cargo_package_name: &str) -> Result<String> {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .into_diagnostic()
        .and_then(|base| xdg_location(&base))
        .map(|path| path.to_string_lossy().into())
}

#[cfg(target_os = "windows")]
fn author_file_path(cargo_package_name: &str) -> Result<String> {
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
fn xdg_location(config_directory: &xdg::BaseDirectories) -> Result<PathBuf> {
    config_directory
        .place_config_file("mit.toml")
        .into_diagnostic()
}

fn from_exec(command: &str) -> Result<String> {
    let commandline = shell_words::split(command).into_diagnostic()?;
    Command::new(commandline.first().unwrap_or(&String::from("")))
        .stderr(Stdio::inherit())
        .args(commandline.iter().skip(1).collect::<Vec<_>>())
        .output()
        .into_diagnostic()
        .and_then(|output| {
            String::from_utf8(output.stdout).map_err(|source| {
                GitMitError::ExecUtf8 {
                    source,
                    command: command.to_string(),
                }
                .into()
            })
        })
}
