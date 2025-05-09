use std::env::current_dir;

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    mit::cmd::get_config_non_clean_behaviour::get_config_non_clean_behaviour, scope::Scope,
};

use crate::get_vcs;

pub fn run(scope: Scope) -> Result<()> {
    let current_dir = current_dir().into_diagnostic()?;
    let vcs = get_vcs(scope == Scope::Local, &current_dir)?;

    let result = get_config_non_clean_behaviour(&vcs)?;
    mit_commit_message_lints::console::style::to_be_piped(&result.to_string());

    Ok(())
}
