use std::env::current_dir;

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    mit::cmd::set_config_rotation::set_config_rotation,
    scope::Scope,
};

use crate::get_vcs;

pub fn run(scope: Scope, rotation: bool) -> Result<()> {
    let current_dir = current_dir().into_diagnostic()?;
    let mut vcs = get_vcs(scope == Scope::Local, &current_dir)?;

    set_config_rotation(&mut vcs, rotation)?;

    Ok(())
}