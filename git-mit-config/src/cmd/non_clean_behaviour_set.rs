use std::env::current_dir;

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    mit::{
        cmd::set_config_non_clean_behaviour::set_config_non_clean_behaviour,
        lib::non_clean_behaviour::BehaviourOption,
    },
    scope::Scope,
};

use crate::get_vcs;

pub fn run(scope: Scope, behaviour: BehaviourOption) -> Result<()> {
    let current_dir = current_dir().into_diagnostic()?;
    let mut vcs = get_vcs(scope == Scope::Local, &current_dir)?;

    set_config_non_clean_behaviour(&mut vcs, behaviour)?;

    Ok(())
}
