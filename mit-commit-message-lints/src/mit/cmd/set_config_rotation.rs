use miette::Result;

use crate::{external::Vcs, mit::lib::rotation_option::RotationOption};

/// Set the rotation configuration setting
///
/// # Errors
///
/// Returns an error if writing to the git config fails.
pub fn set_config_rotation(store: &mut dyn Vcs, rotation: RotationOption) -> Result<()> {
    store.set_str(super::CONFIG_KEY_ROTATION, &rotation.to_string())
}
