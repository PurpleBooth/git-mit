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
