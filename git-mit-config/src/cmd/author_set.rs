use std::env::current_dir;

use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{
    mit::{set_config_authors, Author},
    scope::Scope,
};

use crate::get_vcs;

pub fn run(
    scope: Scope,
    initial: &str,
    name: String,
    email: String,
    signingkey: Option<String>,
) -> Result<()> {
    let current_dir = current_dir().into_diagnostic()?;
    let mut vcs = get_vcs(Scope::Local == scope, &current_dir)?;
    set_config_authors(
        &mut vcs,
        initial,
        &Author::new(
            name.into(),
            email.into(),
            signingkey.map(std::convert::Into::into),
        ),
    )?;

    Ok(())
}
