use std::{convert::TryFrom, env};

use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use cli::{app, args::Args};
use git2::Repository;
use mit_commit_message_lints::{console::style, external::Git2};

mod cli;
mod errors;
use miette::{GraphicalTheme, IntoDiagnostic, Result};
use mit_commit_message_lints::{
    console::style::print_completions,
    relates::{set_relates_to, RelateTo},
};
use mit_commit_message_lints::console::style::miette_install;

fn main() -> Result<()> {
    miette_install();

    let mut app = app::app();
    let args: Args = app.clone().get_matches().into();

    // Simply print and exit if completion option is given.
    if let Some(completion) = args.completion() {
        match completion {
            "bash" => print_completions::<Bash>(&mut app),
            "elvish" => print_completions::<Elvish>(&mut app),
            "fish" => print_completions::<Fish>(&mut app),
            "powershell" => print_completions::<PowerShell>(&mut app),
            "zsh" => print_completions::<Zsh>(&mut app),
            _ => println!("Unknown completion"), // Never reached
        }

        std::process::exit(0);
    }

    let relates_to = args.issue_number()?;

    if repo_present() && !is_hook_present() {
        not_setup_warning();
    };

    let current_dir = env::current_dir().into_diagnostic()?;
    let mut vcs = Git2::try_from(current_dir)?;
    set_relates_to(&mut vcs, &RelateTo::new(relates_to), args.timeout()?)?;

    Ok(())
}

fn not_setup_warning() {
    style::warning("Hooks not found in this repository, your commits won't contain trailers, and lints will not be checked", Some("`git mit-install` `will fix this"));
}

fn is_hook_present() -> bool {
    env::current_dir()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .map(|repo| repo.path().join("hooks").join("commit-msg"))
        .filter(|path_buf| match path_buf.canonicalize().ok() {
            None => false,
            Some(path) => path.to_string_lossy().contains("mit-commit-msg"),
        })
        .is_some()
}

fn repo_present() -> bool {
    env::current_dir()
        .ok()
        .and_then(|path| Repository::discover(path).ok())
        .is_some()
}
