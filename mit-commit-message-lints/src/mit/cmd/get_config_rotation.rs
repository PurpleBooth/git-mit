use miette::Result;

use crate::{external::Vcs, mit::lib::rotation_option::RotationOption};

/// Get the rotation configuration setting
///
/// Returns `None` when rotation is not configured (rotation is off).
/// Returns `Some(RotationOption::RoundRobin)` when round-robin rotation
/// is enabled.
///
/// # Errors
///
/// Returns an error if reading the git config fails, or if the stored
/// value cannot be parsed as a valid rotation option.
pub fn get_config_rotation(store: &dyn Vcs) -> Result<Option<RotationOption>> {
    match store.get_str(super::CONFIG_KEY_ROTATION)?.map(String::from) {
        Some(s) => Ok(Some(s.parse()?)),
        None => Ok(None),
    }
}
