use std::path::{Path, PathBuf};
use std::{env, fs};

use git2::{Config, Error};
use indoc::indoc;

use errors::GitMitInstallError;

pub(crate) use crate::cli::app;
use crate::cli::args::Args;

mod cli;
mod errors;

fn main() -> Result<(), GitMitInstallError> {
    let args: Args = app::app().get_matches().into();

    let hooks = if args.scope().is_global() {
        setup_global_hooks_dir()?
    } else {
        get_local_hooks_dir()?
    };

    if !hooks.exists() {
        fs::create_dir(&hooks)?;
    }

    install_hook(&hooks, "prepare-commit-msg")?;
    install_hook(&hooks, "pre-commit")?;
    install_hook(&hooks, "commit-msg")?;

    if args.scope().is_global() {
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
