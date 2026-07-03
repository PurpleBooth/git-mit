//! Get whether the author trailer (Co-authored-by) is enabled
use miette::Result;

use crate::external::Vcs;

const CONFIG_KEY_AUTHOR_STATUS: &str = "mit.author.enabled";

/// Get whether the author bit (Co-authored-by trailer) is enabled.
///
/// Defaults to `true` when not set, meaning authors are added to commits.
/// When set to `false`, the prepare-commit-msg hook will not append
/// Co-authored-by trailers.
///
/// # Errors
///
/// Returns an error if reading the git config fails.
pub fn get_config_author_status(store: &dyn Vcs) -> Result<bool> {
    Ok(store
        .get_bool(CONFIG_KEY_AUTHOR_STATUS)?
        .unwrap_or(true))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::external::InMemory;

    #[test]
    fn get_config_author_status_defaults_to_true_when_not_set() {
        let mut buffer = BTreeMap::new();
        let vcs_config = InMemory::new(&mut buffer);

        let result = super::get_config_author_status(&vcs_config);

        assert!(
            result.unwrap(),
            "Expected author status to default to true when not set"
        );
    }

    #[test]
    fn get_config_author_status_returns_true_when_set_to_true() {
        let mut buffer = BTreeMap::new();
        buffer.insert("mit.author.enabled".into(), "true".into());
        let vcs_config = InMemory::new(&mut buffer);

        let result = super::get_config_author_status(&vcs_config);

        assert!(
            result.unwrap(),
            "Expected author status to be true when set to 'true'"
        );
    }

    #[test]
    fn get_config_author_status_returns_false_when_set_to_false() {
        let mut buffer = BTreeMap::new();
        buffer.insert("mit.author.enabled".into(), "false".into());
        let vcs_config = InMemory::new(&mut buffer);

        let result = super::get_config_author_status(&vcs_config);

        assert!(
            !result.unwrap(),
            "Expected author status to be false when set to 'false'"
        );
    }
}
