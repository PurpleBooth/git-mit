use miette::Result;
use mit_commit_message_lints::{external::Vcs, scope::Scope};

use crate::{current_dir, get_vcs};

pub fn run(scope: Scope, template: &str) -> Result<()> {
    let current_dir = current_dir()?;
    let mut vcs = get_vcs(scope == Scope::Local, &current_dir)?;

    vcs.set_str("mit.relate.template", template)?;

    Ok(())
}
