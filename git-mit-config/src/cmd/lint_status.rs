use miette::Result;
use mit_commit_message_lints::{external, lints::read_from_toml_or_else_vcs, scope::Scope};
use mit_lint::Lint;

use crate::{current_dir, get_vcs};

pub fn run(scope: Scope, lints: Vec<Lint>) -> Result<()> {
    let current_dir = current_dir()?;
    let vcs = get_vcs(scope == Scope::Local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;

    let config = read_from_toml_or_else_vcs(&toml, &vcs)?;

    mit_commit_message_lints::console::style::lint_table(&lints.into(), &config);

    Ok(())
}
