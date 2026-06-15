use std::collections::BTreeMap;

use miette::Result;

use crate::external::InMemory;

#[test]
fn get_config_rotation_returns_false_when_not_set() -> Result<()> {
    let mut buffer = BTreeMap::new();
    let vcs_config = InMemory::new(&mut buffer);

    // get_config_rotation should return false when not configured
    let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);

    assert_eq!(result.unwrap(), false);

    Ok(())
}

#[test]
fn get_config_rotation_returns_true_when_set_to_true() -> Result<()> {
    let mut buffer = BTreeMap::new();
    buffer.insert("mit.author.rotate".into(), "true".into());
    let vcs_config = InMemory::new(&mut buffer);

    let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);

    assert_eq!(result.unwrap(), true);

    Ok(())
}

#[test]
fn get_config_rotation_returns_false_when_set_to_false() -> Result<()> {
    let mut buffer = BTreeMap::new();
    buffer.insert("mit.author.rotate".into(), "false".into());
    let vcs_config = InMemory::new(&mut buffer);

    let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);

    assert_eq!(result.unwrap(), false);

    Ok(())
}
