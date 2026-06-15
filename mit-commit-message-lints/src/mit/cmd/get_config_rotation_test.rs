use std::collections::BTreeMap;

use miette::Result;

use crate::external::InMemory;
use crate::mit::lib::rotation_option::RotationOption;

#[test]
fn get_config_rotation_returns_none_when_not_set() -> Result<()> {
    let mut buffer = BTreeMap::new();
    let vcs_config = InMemory::new(&mut buffer);

    let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);

    assert_eq!(result.unwrap(), None);

    Ok(())
}

#[test]
fn get_config_rotation_returns_round_robin_when_set() -> Result<()> {
    let mut buffer = BTreeMap::new();
    buffer.insert("mit.author.rotate".into(), "round-robin".into());
    let vcs_config = InMemory::new(&mut buffer);

    let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);

    assert_eq!(result.unwrap(), Some(RotationOption::RoundRobin));

    Ok(())
}

#[test]
fn get_config_rotation_returns_error_for_invalid_value() -> Result<()> {
    let mut buffer = BTreeMap::new();
    buffer.insert("mit.author.rotate".into(), "nonsense".into());
    let vcs_config = InMemory::new(&mut buffer);

    let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);

    assert!(result.is_err());

    Ok(())
}
