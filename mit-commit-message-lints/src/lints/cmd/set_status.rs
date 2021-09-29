use miette::Result;
use mit_lint::Lints;

use crate::external::Vcs;
/// # Errors
///
/// Errors if writing to the VCS config fails
pub fn set_status(lints: Lints, vcs: &mut dyn Vcs, status: bool) -> Result<()> {
    lints
        .config_keys()
        .into_iter()
        .try_for_each(|lint| vcs.set_str(&lint, &status.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests_can_enable_lints_via_a_command {
    use std::collections::{BTreeMap, BTreeSet};

    use mit_lint::{Lint, Lints};

    use crate::{external::InMemory, lints::cmd::set_status::set_status};

    #[test]
    fn we_can_enable_lints() {
        let mut strings = BTreeMap::new();
        strings.insert("mit.lint.pivotal-tracker-id-missing".into(), "false".into());
        let mut config = InMemory::new(&mut strings);

        let mut lints = BTreeSet::new();
        lints.insert(Lint::PivotalTrackerIdMissing);

        set_status(Lints::new(lints), &mut config, true).unwrap();

        let expected = "true".to_string();
        let actual = strings
            .get("mit.lint.pivotal-tracker-id-missing")
            .unwrap()
            .clone();
        assert_eq!(expected, actual);
    }

    #[test]
    fn we_can_disable_lints() {
        let mut strings = BTreeMap::new();
        strings.insert("mit.lint.pivotal-tracker-id-missing".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        let mut lints = BTreeSet::new();
        lints.insert(Lint::PivotalTrackerIdMissing);

        set_status(Lints::new(lints), &mut config, false).unwrap();

        let expected = "false".to_string();
        let actual = strings
            .get("mit.lint.pivotal-tracker-id-missing")
            .unwrap()
            .clone();
        assert_eq!(expected, actual);
    }
}
