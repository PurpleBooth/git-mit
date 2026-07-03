use std::env::current_dir;

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    mit::cmd::set_config_author_status::set_config_author_status, scope::Scope,
};

use crate::get_vcs;

pub fn run(scope: Scope, enabled: bool) -> Result<()> {
    let current_dir = current_dir().into_diagnostic()?;
    let mut vcs = get_vcs(scope == Scope::Local, &current_dir)?;

    set_config_author_status(&mut vcs, enabled)?;

    Ok(())
}
