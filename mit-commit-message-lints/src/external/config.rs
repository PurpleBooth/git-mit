use std::{fs, path::PathBuf};

use git2::Repository;
use miette::{IntoDiagnostic, Result, WrapErr};

/// Find and read the correct toml config
///
/// # Errors
///
/// If we can't find a git repository, or if reading the toml file works
pub fn read_toml(path: PathBuf) -> Result<String> {
    let repository = Repository::discover(path)
        .into_diagnostic()
        .wrap_err("failed to work out location of repository")?;
    let path = repository.path();
    let bare = path.parent().unwrap_or(path).join(".git-mit.toml");
    let dist = path.parent().unwrap_or(path).join(".git-mit.toml.dist");

    if bare.exists() {
        return fs::read_to_string(bare).into_diagnostic();
    }

    if dist.exists() {
        return fs::read_to_string(dist).into_diagnostic();
    }

    Ok(String::new())
}
