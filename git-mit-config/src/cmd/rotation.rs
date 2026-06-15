use std::env::current_dir;

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{mit::cmd::get_config_rotation::get_config_rotation, scope::Scope};

use crate::get_vcs;

pub fn run(scope: Scope) -> Result<()> {
    let current_dir = current_dir().into_diagnostic()?;
    let vcs = get_vcs(scope == Scope::Local, &current_dir)?;

    let rotation = get_config_rotation(&vcs)?;

    println!("{rotation}");

    Ok(())
}
