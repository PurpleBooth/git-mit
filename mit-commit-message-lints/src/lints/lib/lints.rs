use crate::external;
use crate::external::Vcs;
use crate::lints::lib::{lint, Lint};
use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{TryFrom, TryInto};
use std::iter::FromIterator;
use std::vec::IntoIter;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Lints {
    lints: BTreeSet<Lint>,
}

impl Lints {
    #[must_use]
    pub fn new(lints: BTreeSet<Lint>) -> Lints {
        Lints { lints }
    }

    #[must_use]
    pub fn names(self) -> Vec<&'static str> {
        self.lints.iter().map(|lint| lint.name()).collect()
    }

    #[must_use]
    pub fn config_keys(self) -> Vec<String> {
        self.lints.iter().map(|lint| lint.config_key()).collect()
    }

    /// # Errors
    ///
    /// If we fail to parse the toml
    pub fn get_from_toml_or_else_vcs(config: &str, vcs: &mut dyn Vcs) -> Result<Lints, Error> {
        let vcs_lints = Lints::try_from_vcs(vcs)?;
        // contains PB  // contains lint // contains config
        let config: BTreeMap<String, BTreeMap<String, BTreeMap<String, bool>>> =
            toml::from_str(config)?;

        let lint_prefix = lint::CONFIG_KEY_PREFIX.split('.').collect::<Vec<_>>();
        let namespace = (*lint_prefix.get(0).unwrap()).to_string();

        let config = match config.get(&namespace) {
            None => return Ok(vcs_lints),
            Some(lints) => lints,
        };

        let group = (*lint_prefix.get(1).unwrap()).to_string();

        let lint_names = match config.get(&group) {
            None => return Ok(vcs_lints),
            Some(lints) => lints,
        };

        let to_add: Lints = lint_names
            .iter()
            .filter_map(|(key, value)| if *value { Some(key.borrow()) } else { None })
            .collect::<Vec<&str>>()
            .try_into()?;

        let to_remove: Lints = lint_names
            .iter()
            .filter_map(|(key, value)| if *value { None } else { Some(key.borrow()) })
            .collect::<Vec<&str>>()
            .try_into()?;

        Ok(vcs_lints.subtract(&to_remove).merge(&to_add))
    }

    /// Create lints from the VCS configuration
    ///
    /// # Errors
    /// If reading from the VCS fails
    pub fn try_from_vcs(config: &mut dyn Vcs) -> Result<Lints, Error> {
        Ok(Lints::new(
            vec![
                get_config_or_default(config, Lint::DuplicatedTrailers, true)?,
                get_config_or_default(config, Lint::PivotalTrackerIdMissing, false)?,
                get_config_or_default(config, Lint::JiraIssueKeyMissing, false)?,
            ]
            .into_iter()
            .flatten()
            .collect(),
        ))
    }

    #[must_use]
    pub fn merge(&self, other: &Lints) -> Lints {
        Lints::new(BTreeSet::from_iter(self.lints.union(&other.lints).cloned()))
    }

    #[must_use]
    pub fn subtract(&self, other: &Lints) -> Lints {
        Lints::new(BTreeSet::from_iter(
            self.lints.difference(&other.lints).cloned(),
        ))
    }
}

impl std::iter::IntoIterator for Lints {
    type Item = Lint;
    type IntoIter = IntoIter<Lint>;

    fn into_iter(self) -> Self::IntoIter {
        self.lints.into_iter().collect::<Vec<_>>().into_iter()
    }
}

impl TryFrom<Vec<&str>> for Lints {
    type Error = Error;

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let lints = value
            .into_iter()
            .try_fold(
                vec![],
                |lints: Vec<Lint>, item_name| -> Result<Vec<Lint>, Error> {
                    let lint = Lint::try_from(item_name)?;

                    Ok(vec![lints, vec![lint]].concat())
                },
            )
            .map(Vec::into_iter)?;

        Ok(Lints::new(BTreeSet::from_iter(lints)))
    }
}

fn get_config_or_default(
    config: &dyn Vcs,
    lint: Lint,
    default: bool,
) -> Result<Option<Lint>, Error> {
    Ok(config
        .get_bool(&lint.config_key())?
        .or(Some(default))
        .filter(|lint_value| lint_value == &true)
        .map(|_| lint))
}

#[cfg(test)]
mod tests {
    use crate::lints::lib::lints::{Error, Lints};
    use indoc::indoc;

    use crate::lints::Lint::{JiraIssueKeyMissing, PivotalTrackerIdMissing};
    use pretty_assertions::assert_eq;

    use std::collections::{BTreeMap, BTreeSet};

    use crate::{external::InMemory, lints::Lint::DuplicatedTrailers};

    use std::convert::TryInto;

    #[test]
    fn it_returns_an_error_if_one_of_the_names_is_wrong() {
        let lints = vec![
            "pivotal-tracker-id-missing",
            "broken",
            "jira-issue-key-missing",
        ];
        let actual: Result<Lints, Error> = lints.try_into();

        assert_eq!(true, actual.is_err());
    }

    #[test]
    fn it_can_construct_itself_from_names() {
        let lints = vec!["pivotal-tracker-id-missing", "jira-issue-key-missing"];

        let mut expected_lints = BTreeSet::new();
        expected_lints.insert(PivotalTrackerIdMissing);
        expected_lints.insert(JiraIssueKeyMissing);

        let expected = Lints::new(expected_lints);
        let actual: Lints = lints.try_into().expect("Lints to have been parsed");

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_can_give_me_an_into_iterator() {
        let mut lints = BTreeSet::new();
        lints.insert(PivotalTrackerIdMissing);
        lints.insert(JiraIssueKeyMissing);
        let input = Lints::new(lints);

        let expected = vec![PivotalTrackerIdMissing, JiraIssueKeyMissing];
        let actual = input.into_iter().collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_can_give_me_the_names() {
        let mut lints = BTreeSet::new();
        lints.insert(PivotalTrackerIdMissing);
        lints.insert(JiraIssueKeyMissing);
        let input = Lints::new(lints);

        let expected = vec![PivotalTrackerIdMissing.name(), JiraIssueKeyMissing.name()];
        let actual = input.names();

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_can_give_me_the_config_keys() {
        let mut lints = BTreeSet::new();
        lints.insert(PivotalTrackerIdMissing);
        lints.insert(JiraIssueKeyMissing);
        let input = Lints::new(lints);

        let expected = vec![
            PivotalTrackerIdMissing.config_key(),
            JiraIssueKeyMissing.config_key(),
        ];
        let actual = input.config_keys();

        assert_eq!(expected, actual);
    }
    #[test]
    fn try_from_toml_defaults() {
        let mut store = BTreeMap::new();
        let mut vcs = InMemory::new(&mut store);

        let actual = Lints::get_from_toml_or_else_vcs(
            indoc!(
                "
                "
            ),
            &mut vcs,
        )
        .expect("Failed to parse toml");

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);

        let expected = Lints::new(lints);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn try_from_toml_adding_single_lint() {
        let mut store = BTreeMap::new();
        let mut vcs = InMemory::new(&mut store);

        let actual = Lints::get_from_toml_or_else_vcs(
            indoc!(
                "
                [mit.lint]
                \"pivotal-tracker-id-missing\" = true
                "
            ),
            &mut vcs,
        )
        .expect("Failed to parse toml");

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);
        lints.insert(PivotalTrackerIdMissing);
        let expected = Lints::new(lints);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn try_from_toml_removing_single_lint() {
        let mut store = BTreeMap::new();
        let mut vcs = InMemory::new(&mut store);

        let actual = Lints::get_from_toml_or_else_vcs(
            indoc!(
                "
                [mit.lint]
                \"duplicated-trailers\" = false
                "
            ),
            &mut vcs,
        )
        .expect("Failed to parse toml");

        let lints = BTreeSet::new();
        let expected = Lints::new(lints);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn try_from_vcs_defaults() {
        let mut strings = BTreeMap::new();
        let mut config = InMemory::new(&mut strings);

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);
        let expected = Lints::new(lints);

        let actual = Lints::try_from_vcs(&mut config).expect("Failed to read lints from VCS");

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn try_from_vcs_disabled_duplicated_trailers() {
        let mut strings = BTreeMap::new();
        strings.insert("mit.lint.duplicated-trailers".into(), "false".into());
        let mut config = InMemory::new(&mut strings);

        let actual = Lints::try_from_vcs(&mut config).expect("Failed to read lints from VCS");
        let expected: Lints = Lints::new(BTreeSet::new());

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn try_from_vcs_enabled_duplicated_trailers() {
        let mut strings = BTreeMap::new();
        strings.insert("mit.lint.duplicated-trailers".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);
        let expected = Lints::new(lints);

        let actual = Lints::try_from_vcs(&mut config).expect("Failed to read lints from VCS");

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn try_from_vcs_enabled_pivotal_tracker_id() {
        let mut strings = BTreeMap::new();
        strings.insert("mit.lint.pivotal-tracker-id-missing".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);
        lints.insert(PivotalTrackerIdMissing);
        let expected = Lints::new(lints);

        let actual = Lints::try_from_vcs(&mut config).expect("Failed to read lints from VCS");

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn try_from_vcs_enabled_jira_issue_key_missing() {
        let mut strings = BTreeMap::new();
        strings.insert("mit.lint.jira-issue-key-missing".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);
        lints.insert(JiraIssueKeyMissing);
        let expected = Lints::new(lints);

        let actual = Lints::try_from_vcs(&mut config).expect("Failed to read lints from VCS");

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn try_from_vcs_disabled_jira_issue_key_missing() {
        let mut strings = BTreeMap::new();
        strings.insert("mit.lint.jira-issue-key-missing".into(), "false".into());
        let mut config = InMemory::new(&mut strings);

        let actual = Lints::try_from_vcs(&mut config).expect("Failed to read lints from VCS");

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);
        let expected = Lints::new(lints);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn two_sets_of_lints_can_be_merged() {
        let mut set_a_lints = BTreeSet::new();
        set_a_lints.insert(PivotalTrackerIdMissing);

        let mut set_b_lints = BTreeSet::new();
        set_b_lints.insert(DuplicatedTrailers);

        let set_a = Lints::new(set_a_lints);
        let set_b = Lints::new(set_b_lints);

        let actual = set_a.merge(&set_b);

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);
        lints.insert(PivotalTrackerIdMissing);
        let expected = Lints::new(lints);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }
    #[test]
    fn when_merging_overlapping_does_not_lead_to_duplication() {
        let mut set_a_lints = BTreeSet::new();
        set_a_lints.insert(PivotalTrackerIdMissing);

        let mut set_b_lints = BTreeSet::new();
        set_b_lints.insert(DuplicatedTrailers);
        set_b_lints.insert(PivotalTrackerIdMissing);

        let set_a = Lints::new(set_a_lints);
        let set_b = Lints::new(set_b_lints);

        let actual = set_a.merge(&set_b);

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);
        lints.insert(PivotalTrackerIdMissing);
        let expected = Lints::new(lints);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }
    #[test]
    fn we_can_subtract_lints_from_the_lint_list() {
        let mut set_a_lints = BTreeSet::new();
        set_a_lints.insert(JiraIssueKeyMissing);
        set_a_lints.insert(PivotalTrackerIdMissing);

        let mut set_b_lints = BTreeSet::new();
        set_b_lints.insert(DuplicatedTrailers);
        set_b_lints.insert(PivotalTrackerIdMissing);

        let set_a = Lints::new(set_a_lints);
        let set_b = Lints::new(set_b_lints);

        let actual = set_a.subtract(&set_b);

        let mut lints = BTreeSet::new();
        lints.insert(JiraIssueKeyMissing);
        let expected = Lints::new(lints);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    LintNameUnknown(#[from] lint::Error),
    #[error("failed to read lint config from git: {0}")]
    VcsIo(#[from] external::Error),
    #[error("Failed to parse lint config file: {0}")]
    TomlParse(#[from] toml::de::Error),
}
