use std::convert::TryInto;

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    console::style::to_be_piped,
    external,
    lints::read_from_toml_or_else_vcs,
    scope::Scope,
};

use crate::{current_dir, get_vcs};

pub fn run(scope: Scope) -> Result<()> {
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(scope == Scope::Local, &current_dir)?;
    let input_toml = external::read_toml(current_dir)?;

    let output_toml: String = read_from_toml_or_else_vcs(&input_toml, &mut vcs)?
        .try_into()
        .into_diagnostic()?;

    to_be_piped(output_toml.trim());

    Ok(())
}
