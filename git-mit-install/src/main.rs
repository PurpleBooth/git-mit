mod cli;
use std::{env, fs, io};

fn main() -> Result<(), GitMitInstallError> {
    cli::app().get_matches();

    let hooks = git2::Repository::discover(env::current_dir()?)?
        .path()
        .join("hooks");

    if !hooks.exists() {
        fs::create_dir(&hooks)?;
    }

    let mit_prepare_commit_msg = which::which("mit-prepare-commit-msg").unwrap();
    std::os::unix::fs::symlink(mit_prepare_commit_msg, &hooks.join("prepare-commit-msg"))?;

    let mit_prepare_commit_msg = which::which("mit-pre-commit").unwrap();
    std::os::unix::fs::symlink(mit_prepare_commit_msg, &hooks.join("pre-commit"))?;

    let mit_prepare_commit_msg = which::which("mit-commit-msg").unwrap();
    std::os::unix::fs::symlink(mit_prepare_commit_msg, &hooks.join("commit-msg"))?;

    Ok(())
}
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitMitInstallError {
    #[error("failed to find git repository: {0}")]
    Git2(#[from] git2::Error),
    #[error("failed to install hooks: {0}")]
    Io(#[from] io::Error),
}
