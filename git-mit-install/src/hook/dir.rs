use std::{env, fs, path::PathBuf};

use git2::Config;
use miette::{IntoDiagnostic, Result};
pub fn create(global: bool) -> Result<PathBuf> {
    let hooks = if global {
        setup_global_hooks_dir()?
    } else {
        get_local_hooks_dir()?
    };

    if !hooks.exists() {
        fs::create_dir(&hooks).into_diagnostic()?;
    }
    Ok(hooks)
}

fn get_local_hooks_dir() -> Result<PathBuf> {
    let current_dir = env::current_dir().into_diagnostic()?;
    let buf = git2::Repository::discover(current_dir)
        .into_diagnostic()?
        .path()
        .join("hooks");
    Ok(buf)
}

fn setup_global_hooks_dir() -> Result<PathBuf> {
    let mut config = git2::Config::open_default().into_diagnostic()?;

    let template_dir = if let Ok(template_dir) = git_template_dir(&mut config) {
        template_dir
    } else {
        let template_dir = new_template_folder();
        config
            .set_str("init.templatedir", template_dir.to_string_lossy().as_ref())
            .into_diagnostic()?;
        template_dir
    };

    let hooks = template_dir.join("hooks");
    fs::create_dir_all(&hooks).into_diagnostic()?;
    Ok(hooks)
}

fn new_template_folder() -> PathBuf {
    PathBuf::from(home_dir())
        .join(".config")
        .join("git")
        .join("init-template")
}

fn git_template_dir(config: &mut Config) -> Result<PathBuf> {
    config
        .snapshot()
        .into_diagnostic()?
        .get_path("init.templatedir")
        .into_diagnostic()
}

#[cfg(not(target_os = "windows"))]
fn home_dir() -> String {
    env!("HOME").into()
}

#[cfg(target_os = "windows")]
fn home_dir() -> String {
    env!("USERPROFILE").into()
}
