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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use miette::Result;

    use crate::external::InMemory;
    use crate::mit::lib::rotation_option::RotationOption;

    #[test]
    fn set_config_rotation_writes_round_robin_and_reads_back() -> Result<()> {
        let mut buffer = BTreeMap::new();
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            crate::mit::cmd::set_config_rotation::set_config_rotation(
                &mut vcs_config,
                RotationOption::RoundRobin,
            )?;
        }

        assert_eq!(
            buffer.get("mit.author.rotate"),
            Some(&"round-robin".to_string())
        );

        let vcs_config = InMemory::new(&mut buffer);
        let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);
        assert_eq!(result.unwrap(), Some(RotationOption::RoundRobin));

        Ok(())
    }
}
