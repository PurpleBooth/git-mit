use miette::Result;

use crate::external::Vcs;

/// Get the rotation configuration setting
///
/// Returns `false` if the rotation setting is not configured.
///
/// # Errors
///
/// Returns an error if reading the git config fails.
pub fn get_config_rotation(store: &dyn Vcs) -> Result<bool> {
    Ok(store.get_bool(super::CONFIG_KEY_ROTATION)?.unwrap_or(false))
}
