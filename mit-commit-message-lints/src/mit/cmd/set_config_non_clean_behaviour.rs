//! Set rebase behavior for author trailers
use miette::Result;

use crate::{external::Vcs, mit::lib::non_clean_behaviour::BehaviourOption};

/// # Errors
///
/// On write failure
pub fn set_config_non_clean_behaviour(
    store: &mut dyn Vcs,
    behaviour: BehaviourOption,
) -> Result<()> {
    store.set_str("mit.author.non-clean-behaviour", &behaviour.to_string())?;

    Ok(())
}
