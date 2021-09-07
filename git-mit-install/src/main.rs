use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub(crate) use crate::cli::app;
use indoc::indoc;
use thiserror::Error;

mod cli;

fn main() -> Result<(), GitMitInstallError> {
    let matches = app::app().get_matches();

    let hooks = if let Some("global") = matches.value_of("scope") {
        let mut config = git2::Config::open_default()?;

        if let Ok(path) = config.snapshot()?.get_path("init.templatedir") {
            let hooks = path.join("hooks");
            fs::create_dir_all(&hooks)?;
            hooks
        } else {
            let init_template = PathBuf::from(home_dir())
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

    if let Some("global") = matches.value_of("scope") {
        mit_commit_message_lints::console::style::success(
            "git-mit will be added for newly created or cloned repositories",
            "inside existing repositories run \"git init\" to set them up",
        );
    } else {
        mit_commit_message_lints::console::style::success(
            "git-mit is setup for the current repository",
            indoc! {r#"
                Optionally you can install git-mit for all new "git clone" and "git init" commands

                git mit-install --scope=global
            "#},
        );
    }

    mit_commit_message_lints::console::style::success(
        "Adding your first pairing partners",
        indoc! {r#"
            git mit-config mit set bt "Billie Thompson" billie@example.com
            git mit-config mit set se "Someone Else" someone@example.com

            To add multiple users to your commit run. Remember to include yourself!

            git mit bt se

            Optionally you can also add a issue number by running

            git mit-relates-to "[#134]"

            When you can use the "-m" flag or your editor, both work as normal

            git commit -m "Your message"

            The authors and issue number appear on the commit. These authors are saved into your current repository for ad-hoc pairing. When you're ready make the authors everywhere run

            mkdir -p "$HOME/.config/git-mit"
            git mit-config generate > "$HOME/.config/git-mit/mit.toml"
        "#},
    );

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn home_dir() -> String {
    env!("HOME").into()
}

#[cfg(target_os = "windows")]
fn home_dir() -> String {
    env!("USERPROFILE").into()
}

fn install_hook(hook_path: &Path, hook_name: &str) -> Result<(), GitMitInstallError> {
    #[cfg(target_os = "windows")]
    let suffix = ".exe";
    #[cfg(not(target_os = "windows"))]
    let suffix = "";
    let binary_path = which::which(format!("mit-{}{}", hook_name, suffix)).unwrap();
    let install_path = hook_path.join(format!("{}{}", hook_name, suffix));
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
