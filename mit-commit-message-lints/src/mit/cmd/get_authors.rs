use std::{
    convert::TryFrom,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use miette::{IntoDiagnostic, Result};

use crate::mit::Authors;

/// From a cli args, get the author configuration
pub trait AuthorArgs {
    /// Get the command to run to generate the authors file
    fn author_command(&self) -> Option<&str>;

    /// Get path to author file
    fn author_file(&self) -> Option<&str>;
}

/// Get authors from config
///
/// # Errors
///
/// miette error on failure of command
pub fn get_authors(args: &dyn AuthorArgs) -> Result<Authors> {
    let toml = match args.author_command() {
        Some(command) => from_exec(command),
        None => from_file(args),
    }?;

    let authors: Authors = Authors::try_from(&*toml)?;
    Ok(authors)
}

fn from_file(args: &dyn AuthorArgs) -> Result<String> {
    match args.author_file() {
        None => Err(super::errors::Error::AuthorFileNotSet.into()),
        Some(path) => Ok(path),
    }
    .and_then(|path| match path {
        "$HOME/.config/git-mit/mit.toml" => author_file_path(),
        _ => Ok(path.into()),
    })
    .map(|path| fs::read_to_string(&path).unwrap_or_default())
}

#[cfg(not(target_os = "windows"))]
fn author_file_path() -> Result<String> {
    let home: PathBuf = std::env::var("HOME").into_diagnostic()?.into();
    return Ok(home
        .join(".config")
        .join("git-mit")
        .join("mit.toml")
        .to_string_lossy()
        .to_string());
}

#[cfg(target_os = "windows")]
fn author_file_path() -> Result<String> {
    std::env::var("APPDATA")
        .map(|x| {
            PathBuf::from(x)
                .join("git-mit")
                .join("mit.toml")
                .to_string_lossy()
                .into()
        })
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
                super::errors::Error::ExecUtf8 {
                    source,
                    command: command.to_string(),
                }
                .into()
            })
        })
}
