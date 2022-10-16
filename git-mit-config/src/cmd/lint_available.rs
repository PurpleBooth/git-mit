use std::env::current_dir;

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{external, lints::read_from_toml_or_else_vcs, scope::Scope};
use mit_lint::Lints;

use crate::get_vcs;

pub fn run(scope: Scope) -> Result<()> {
    let current_dir = current_dir().into_diagnostic()?;
    let mut vcs = get_vcs(scope == Scope::Local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;

    let lints = read_from_toml_or_else_vcs(&toml, &mut vcs)?;
    mit_commit_message_lints::console::style::lint_table(Lints::available(), &lints);

    Ok(())
}
