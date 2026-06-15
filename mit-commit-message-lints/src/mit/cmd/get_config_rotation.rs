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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::external::InMemory;
    use crate::mit::lib::rotation_option::RotationOption;

    #[test]
    fn get_config_rotation_returns_none_when_not_set() {
        let mut buffer = BTreeMap::new();
        let vcs_config = InMemory::new(&mut buffer);

        let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);

        assert_eq!(
            result.unwrap(),
            None,
            "Expected no rotation config when the key is not set"
        );
    }

    #[test]
    fn get_config_rotation_returns_round_robin_when_set() {
        let mut buffer = BTreeMap::new();
        buffer.insert("mit.author.rotate".into(), "round-robin".into());
        let vcs_config = InMemory::new(&mut buffer);

        let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);

        assert_eq!(
            result.unwrap(),
            Some(RotationOption::RoundRobin),
            "Expected round-robin rotation config when set to 'round-robin'"
        );
    }

    #[test]
    fn get_config_rotation_returns_error_for_invalid_value() {
        let mut buffer = BTreeMap::new();
        buffer.insert("mit.author.rotate".into(), "nonsense".into());
        let vcs_config = InMemory::new(&mut buffer);

        let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);

        assert!(
            result.is_err(),
            "Expected an error when rotation config is set to an invalid value"
        );
    }
}
