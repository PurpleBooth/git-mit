use std::convert::TryFrom;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use mit_commit_message_lints::mit::Authors;

use crate::cli::args::Args;
use crate::errors::GitMitError;

pub(crate) fn load(args: &Args) -> Result<Authors, GitMitError> {
    let toml = match args.command() {
        Some(command) => from_exec(command),
        None => from_file(args),
    }?;

    let authors: Authors = Authors::try_from(&*toml)?;
    Ok(authors)
}

fn from_file(args: &Args) -> Result<String, GitMitError> {
    args.author_file()
        .ok_or(GitMitError::AuthorFileNotSet)
        .and_then(|path| match path {
            "$HOME/.config/git-mit/mit.toml" => author_file_path(env!("CARGO_PKG_NAME")),
            _ => Ok(path.into()),
        })
        .and_then(|path| {
            fs::read_to_string(&path).map_err(|error| GitMitError::new_io(path, &error))
        })
}

#[cfg(not(target_os = "windows"))]
fn author_file_path(cargo_package_name: &str) -> Result<String, GitMitError> {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(GitMitError::from)
        .and_then(|base| xdg_location(&base))
        .map(|path| path.to_string_lossy().into())
}

#[cfg(target_os = "windows")]
fn author_file_path(cargo_package_name: &str) -> Result<String, GitMitError> {
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
fn xdg_location(config_directory: &xdg::BaseDirectories) -> Result<PathBuf, GitMitError> {
    config_directory
        .place_config_file("mit.toml")
        .map_err(|error| GitMitError::new_io("<config_dir>/mit.toml".into(), &error))
}

fn from_exec(command: &str) -> Result<String, GitMitError> {
    let commandline = shell_words::split(command)?;
    Command::new(commandline.first().unwrap_or(&String::from("")))
        .stderr(Stdio::inherit())
        .args(commandline.iter().skip(1).collect::<Vec<_>>())
        .output()
        .map_err(|error| GitMitError::new_exec(command.into(), &error))
        .and_then(|output| String::from_utf8(output.stdout).map_err(GitMitError::from))
}
