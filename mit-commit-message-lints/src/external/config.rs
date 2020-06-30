use std::fs;
use std::path::PathBuf;

use git2::Repository;

use crate::external::Error;

/// Find and read the correct toml config
///
/// # Errors
///
/// If we can't find a git repository, or if reading the toml file works
pub fn read_toml(path: PathBuf) -> Result<String, Error> {
    let repository = Repository::discover(path)?;
    let path = repository.path();
    let bare = path.parent().unwrap_or(path).join(".git-mit.toml");
    let dist = path.parent().unwrap_or(path).join(".git-mit.toml.dist");

    if bare.exists() {
        return Ok(fs::read_to_string(bare)?);
    }

    if dist.exists() {
        return Ok(fs::read_to_string(dist)?);
    }

    Ok("".to_string())
}
