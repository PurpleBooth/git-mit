//! Set whether the author trailer (Co-authored-by) is enabled
use miette::Result;

use crate::external::Vcs;

const CONFIG_KEY_AUTHOR_STATUS: &str = "mit.author.enabled";

/// Set whether the author bit (Co-authored-by trailer) is enabled.
///
/// When set to `false`, the prepare-commit-msg hook will not append
/// Co-authored-by trailers to commit messages.
///
/// # Errors
///
/// Returns an error if writing to the git config fails.
pub fn set_config_author_status(store: &mut dyn Vcs, enabled: bool) -> Result<()> {
    store.set_str(CONFIG_KEY_AUTHOR_STATUS, &enabled.to_string())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::external::InMemory;

    #[test]
    fn set_config_author_status_writes_true_and_reads_back() {
        let mut buffer = BTreeMap::new();
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            super::set_config_author_status(&mut vcs_config, true).unwrap();
        }

        assert_eq!(
            buffer.get("mit.author.enabled"),
            Some(&"true".to_string()),
            "Expected the author status config to be set to 'true'"
        );
    }

    #[test]
    fn set_config_author_status_writes_false_and_reads_back() {
        let mut buffer = BTreeMap::new();
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            super::set_config_author_status(&mut vcs_config, false).unwrap();
        }

        assert_eq!(
            buffer.get("mit.author.enabled"),
            Some(&"false".to_string()),
            "Expected the author status config to be set to 'false'"
        );
    }

    #[test]
    fn set_config_author_status_round_trips_through_get() {
        let mut buffer = BTreeMap::new();
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            super::set_config_author_status(&mut vcs_config, false).unwrap();
        }

        let vcs_config = InMemory::new(&mut buffer);
        let result =
            crate::mit::cmd::get_config_author_status::get_config_author_status(&vcs_config);
        assert!(
            !result.unwrap(),
            "Expected author status to be false after setting it to false"
        );
    }
}
