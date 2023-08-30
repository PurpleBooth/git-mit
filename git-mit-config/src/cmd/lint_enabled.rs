use miette::Result;
use mit_commit_message_lints::{external, lints::read_from_toml_or_else_vcs, scope::Scope};

use crate::{current_dir, get_vcs};

pub fn run(matches: Scope) -> Result<()> {
    let current_dir = current_dir()?;
    let vcs = get_vcs(matches == Scope::Local, &current_dir)?;
    let toml = external::read_toml(current_dir)?;

    let lints = read_from_toml_or_else_vcs(&toml, &vcs)?;
    mit_commit_message_lints::console::style::lint_table(&lints, &lints);

    Ok(())
}
