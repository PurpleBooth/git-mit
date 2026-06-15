use miette::Result;

use crate::external::Vcs;

/// Set the rotation configuration setting
///
/// # Errors
///
/// Returns an error if writing to the git config fails.
pub fn set_config_rotation(store: &mut dyn Vcs, rotate: bool) -> Result<()> {
    store.set_str(super::CONFIG_KEY_ROTATION, &rotate.to_string())
}