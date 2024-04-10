use std::env::current_dir;

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{external, scope::Scope};
use mit_lint::Lint;

use crate::get_vcs;

pub fn run(scope: Scope, lints: Vec<Lint>) -> Result<()> {
    let current_dir = current_dir().into_diagnostic()?;
    let mut vcs = get_vcs(scope == Scope::Local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;
    if !toml.is_empty() {
        mit_commit_message_lints::console::style::warning(
            "Warning: your config is overridden by a repository config file",
            None,
        );
    }

    mit_commit_message_lints::lints::set_status(lints.into(), &mut vcs, false)?;

    Ok(())
}
