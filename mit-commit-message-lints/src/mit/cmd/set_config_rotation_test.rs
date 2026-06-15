use std::collections::BTreeMap;

use miette::Result;

use crate::external::InMemory;

#[test]
fn set_config_rotation_writes_true_and_reads_back() -> Result<()> {
    let mut buffer = BTreeMap::new();
    let mut vcs_config = InMemory::new(&mut buffer);

    crate::mit::cmd::set_config_rotation::set_config_rotation(&mut vcs_config, true)?;

    assert_eq!(buffer.get("mit.author.rotate"), Some(&"true".to_string()));

    let vcs_config = InMemory::new(&mut buffer);
    let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);
    assert_eq!(result.unwrap(), true);

    Ok(())
}

#[test]
fn set_config_rotation_writes_false_and_reads_back() -> Result<()> {
    let mut buffer = BTreeMap::new();
    let mut vcs_config = InMemory::new(&mut buffer);

    crate::mit::cmd::set_config_rotation::set_config_rotation(&mut vcs_config, false)?;

    assert_eq!(buffer.get("mit.author.rotate"), Some(&"false".to_string()));

    let vcs_config = InMemory::new(&mut buffer);
    let result = crate::mit::cmd::get_config_rotation::get_config_rotation(&vcs_config);
    assert_eq!(result.unwrap(), false);

    Ok(())
}