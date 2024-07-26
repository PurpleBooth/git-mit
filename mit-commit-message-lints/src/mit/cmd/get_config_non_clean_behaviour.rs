//! Get rebase behavior for author trailers
use miette::Result;

use crate::{external::Vcs, mit::lib::non_clean_behaviour::BehaviourOption};

/// # Errors
///
/// On failure to parse the behaviour from the git config
pub fn get_config_non_clean_behaviour(store: &dyn Vcs) -> Result<BehaviourOption> {
    let behaviour_opt = store.get_str("mit.author.non-clean-behaviour")?;

    if let Some(behaviour_str) = behaviour_opt {
        let behaviour: BehaviourOption = behaviour_str.parse()?;

        return Ok(behaviour);
    };
    Ok(BehaviourOption::AddTo)
}
