use std::{env, fs, path::PathBuf};

use git2::{Config, Error};

use crate::errors::GitMitInstallError;

pub fn create(global: bool) -> Result<PathBuf, GitMitInstallError> {
    let hooks = if global {
        setup_global_hooks_dir()?
    } else {
        get_local_hooks_dir()?
    };

    if !hooks.exists() {
        fs::create_dir(&hooks)?;
    }
    Ok(hooks)
}

fn get_local_hooks_dir() -> Result<PathBuf, GitMitInstallError> {
    let current_dir = env::current_dir()?;
    let buf = git2::Repository::discover(current_dir)?
        .path()
        .join("hooks");
    Ok(buf)
}

fn setup_global_hooks_dir() -> Result<PathBuf, GitMitInstallError> {
    let mut config = git2::Config::open_default()?;

    let template_dir = if let Ok(template_dir) = git_template_dir(&mut config) {
        template_dir
    } else {
        let template_dir = new_template_folder();
        config.set_str("init.templatedir", &template_dir.to_string_lossy())?;
        template_dir
    };

    let hooks = template_dir.join("hooks");
    fs::create_dir_all(&hooks)?;
    Ok(hooks)
}

fn new_template_folder() -> PathBuf {
    PathBuf::from(home_dir())
        .join(".config")
        .join("git")
        .join("init-template")
}

fn git_template_dir(config: &mut Config) -> Result<PathBuf, Error> {
    config.snapshot()?.get_path("init.templatedir")
}

#[cfg(not(target_os = "windows"))]
fn home_dir() -> String {
    env!("HOME").into()
}

#[cfg(target_os = "windows")]
fn home_dir() -> String {
    env!("USERPROFILE").into()
}
