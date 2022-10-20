use std::{
    env,
    fs,
    path::{Path, PathBuf},
};

use git2::Config;
use miette::{IntoDiagnostic, Result};
pub fn create(global: bool, home_dir: &Path) -> Result<PathBuf> {
    let hooks = if global {
        setup_global_hooks_dir(home_dir)?
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
    let repository = git2::Repository::discover(current_dir).into_diagnostic()?;
    let config = repository.config().into_diagnostic()?;
    let default_path = repository.path().join("hooks");
    let buf = config.get_path("core.hooksPath").unwrap_or(default_path);
    Ok(buf)
}

fn setup_global_hooks_dir(home_dir: &Path) -> Result<PathBuf> {
    let mut config = git2::Config::open_default().into_diagnostic()?;

    let template_dir = if let Ok(template_dir) = git_template_dir(&mut config) {
        template_dir
    } else {
        let template_dir = new_template_folder(home_dir);
        config
            .set_str("init.templatedir", template_dir.to_string_lossy().as_ref())
            .into_diagnostic()?;
        template_dir
    };

    let hooks = template_dir.join("hooks");
    fs::create_dir_all(&hooks).into_diagnostic()?;
    Ok(hooks)
}

fn new_template_folder(home_dir: &Path) -> PathBuf {
    home_dir.join(".config").join("git").join("init-template")
}

fn git_template_dir(config: &mut Config) -> Result<PathBuf> {
    config
        .snapshot()
        .into_diagnostic()?
        .get_path("init.templatedir")
        .into_diagnostic()
}
