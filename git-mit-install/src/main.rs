use std::{env, fs, io};

use std::path::{Path, PathBuf};
use thiserror::Error;

mod cli;

fn main() -> Result<(), GitMitInstallError> {
    let matches = cli::app().get_matches();

    let hooks = if let Some("global") = matches.value_of("scope") {
        let mut config = git2::Config::open_default()?;

        if let Ok(path) = config.snapshot()?.get_path("init.templatedir") {
            let hooks = path.join("hooks");
            fs::create_dir_all(&hooks)?;
            hooks
        } else {
            let init_template = PathBuf::from(env!("HOME"))
                .join(".config")
                .join("git")
                .join("init-template");
            let hooks = init_template.join("hooks");
            fs::create_dir_all(&hooks)?;
            config.set_str("init.templatedir", "~/.config/git/init-template")?;
            hooks
        }
    } else {
        git2::Repository::discover(env::current_dir()?)?
            .path()
            .join("hooks")
    };

    if !hooks.exists() {
        fs::create_dir(&hooks)?;
    }

    install_hook(&hooks, "prepare-commit-msg")?;
    install_hook(&hooks, "pre-commit")?;
    install_hook(&hooks, "commit-msg")?;
    Ok(())
}

fn install_hook(hook_path: &Path, hook_name: &str) -> Result<(), GitMitInstallError> {
    let binary_path = which::which(format!("mit-{}", hook_name)).unwrap();
    let install_path = hook_path.join(hook_name);
    let install_path_destination = install_path.read_link();
    if let Ok(existing_hook_path) = install_path_destination.and_then(|x| x.canonicalize()) {
        if existing_hook_path == install_path {
            return Ok(());
        }
    }

    if install_path.exists() {
        let mut tip = format!("Couldn't create hook at {}, it already exists, you need to remove this before continuing", install_path.to_string_lossy());
        if let Ok(dest) = install_path.read_link() {
            tip = format!(
                "{}\nlooks like it's a symlink to {}",
                tip,
                dest.to_string_lossy()
            );
        }

        mit_commit_message_lints::console::style::problem("couldn't install hook", &tip);

        return Err(GitMitInstallError::ExistingHook);
    }

    link(binary_path, install_path)?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn link(binary_path: PathBuf, install_path: PathBuf) -> Result<(), GitMitInstallError> {
    std::os::unix::fs::symlink(binary_path, install_path)?;

    Ok(())
}

#[cfg(target_os = "windows")]
fn link(binary_path: PathBuf, install_path: PathBuf) -> Result<(), GitMitInstallError> {
    std::os::windows::fs::symlink_file(binary_path, install_path)?;

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
