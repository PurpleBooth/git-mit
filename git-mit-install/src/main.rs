use std::{env, fs, io};

use std::path::PathBuf;
use thiserror::Error;

mod cli;

fn main() -> Result<(), GitMitInstallError> {
    cli::app().get_matches();

    let hooks = git2::Repository::discover(env::current_dir()?)?
        .path()
        .join("hooks");

    if !hooks.exists() {
        fs::create_dir(&hooks)?;
    }

    install_hook(&hooks, "prepare-commit-msg")?;
    install_hook(&hooks, "pre-commit")?;
    install_hook(&hooks, "commit-msg")?;
    Ok(())
}

fn install_hook(hook_path: &PathBuf, hook_name: &str) -> Result<(), GitMitInstallError> {
    let binary_path = which::which(format!("mit-{}", hook_name)).unwrap();
    let install_path = hook_path.join(hook_name);
    let install_path_destination = install_path.read_link();
    if let Ok(existing_hook_path) = install_path_destination.and_then(|x| x.canonicalize()) {
        if existing_hook_path == install_path {
            return Ok(());
        }
    }

    if install_path.exists() {
        eprintln!("Couldn't create hook at {}, it already exists, you need to remove this before continuing", install_path.to_string_lossy());

        if let Ok(dest) = install_path.read_link() {
            eprintln!("looks like it's a symlink to {}", dest.to_string_lossy());
        }

        return Err(GitMitInstallError::ExistingHook);
    }

    std::os::unix::fs::symlink(binary_path, &install_path)?;

    Ok(())
}

#[derive(Error, Debug)]
pub enum GitMitInstallError {
    #[error("failed install hook")]
    ExistingHook,
    #[error("failed to find git repository: {0}")]
    Git2(#[from] git2::Error),
    #[error("failed to install hooks: {0}")]
    Io(#[from] io::Error),
}
