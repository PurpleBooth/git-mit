use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet};
use std::convert::{TryFrom, TryInto};

use std::vec::IntoIter;

use thiserror::Error;

use crate::external;
use crate::external::Vcs;
use crate::lints::lib::{lint, Lint};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Lints {
    lints: BTreeSet<Lint>,
}

lazy_static! {
    static ref AVAILABLE: Lints = {
        let set = Lint::iterator().collect();
        Lints::new(set)
    };
}

impl Lints {
    #[must_use]
    pub fn new(lints: BTreeSet<Lint>) -> Lints {
        Lints { lints }
    }

    #[must_use]
    pub fn available() -> &'static Lints {
        &AVAILABLE
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
            Lint::iterator()
                .flat_map(|lint| {
                    get_config_or_default(config, lint, lint.enabled_by_default()).transpose()
                })
                .collect::<Result<BTreeSet<Lint>, Error>>()?,
        ))
    }

    #[must_use]
    pub fn merge(&self, other: &Lints) -> Lints {
        Lints::new(self.lints.union(&other.lints).cloned().collect())
    }

    #[must_use]
    pub fn subtract(&self, other: &Lints) -> Lints {
        Lints::new(self.lints.difference(&other.lints).cloned().collect())
    }
}

impl std::iter::IntoIterator for Lints {
    type Item = Lint;
    type IntoIter = IntoIter<Lint>;

    fn into_iter(self) -> Self::IntoIter {
        self.lints.into_iter().collect::<Vec<_>>().into_iter()
    }
}

impl TryFrom<Lints> for String {
    type Error = Error;

    fn try_from(lints: Lints) -> Result<Self, Self::Error> {
        let enabled: Vec<_> = lints.into();

        let config: BTreeMap<String, bool> = Lint::iterator()
            .map(|x| (x, enabled.contains(&x)))
            .fold(BTreeMap::new(), |mut acc, (lint, state)| {
                acc.insert(lint.to_string(), state);
                acc
            });

        let mut inner: BTreeMap<String, BTreeMap<String, bool>> = BTreeMap::new();
        inner.insert("lint".into(), config);
        let mut output: BTreeMap<String, BTreeMap<String, BTreeMap<String, bool>>> =
            BTreeMap::new();
        output.insert("mit".into(), inner);

        Ok(toml::to_string(&output)?)
    }
}

impl From<Vec<Lint>> for Lints {
    fn from(lints: Vec<Lint>) -> Self {
        Lints::new(lints.into_iter().collect())
    }
}

impl From<Lints> for Vec<Lint> {
    fn from(lints: Lints) -> Self {
        lints.into_iter().collect()
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

        Ok(Lints::new(lints.collect()))
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
    use std::collections::{BTreeMap, BTreeSet};
    use std::convert::{TryFrom, TryInto};

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::lints::lib::lint::Lint::{
        BodyWiderThan72Characters, GitHubIdMissing, SubjectLongerThan72Characters,
        SubjectNotSeparateFromBody,
    };
    use crate::lints::lib::lints::{Error, Lints};
    use crate::lints::Lint;
    use crate::lints::Lint::{JiraIssueKeyMissing, PivotalTrackerIdMissing};
    use crate::{external::InMemory, lints::Lint::DuplicatedTrailers};

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
    fn it_can_convert_into_a_vec() {
        let mut lints = BTreeSet::new();
        lints.insert(PivotalTrackerIdMissing);
        lints.insert(JiraIssueKeyMissing);
        let input = Lints::new(lints);

        let expected = vec![PivotalTrackerIdMissing, JiraIssueKeyMissing];
        let actual: Vec<Lint> = input.into();

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
    fn can_get_all() {
        let actual = Lints::available();
        let lints = Lint::iterator().collect();
        let expected = &Lints::new(lints);

        assert_eq!(
            expected, actual,
            "Expected all the lints to be {:?}, instead got {:?}",
            expected, actual
        )
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
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);

        let expected = Lints::new(lints);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn get_toml() {
        let mut store = BTreeMap::new();
        let _vcs = InMemory::new(&mut store);

        let mut lints_on = BTreeSet::new();
        lints_on.insert(DuplicatedTrailers);
        lints_on.insert(SubjectNotSeparateFromBody);
        lints_on.insert(SubjectLongerThan72Characters);
        lints_on.insert(BodyWiderThan72Characters);
        lints_on.insert(PivotalTrackerIdMissing);
        let actual = String::try_from(Lints::new(lints_on)).expect("Failed to serialise");
        let expected = indoc!(
            "
            [mit.lint]
            body-wider-than-72-characters = true
            duplicated-trailers = true
            github-id-missing = false
            jira-issue-key-missing = false
            not-conventional-commit = false
            not-emoji-log = false
            pivotal-tracker-id-missing = true
            subject-line-ends-with-period = false
            subject-line-not-capitalized = false
            subject-longer-than-72-characters = true
            subject-not-separated-from-body = true
            "
        );

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
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);
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

        let mut lints = BTreeSet::new();
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);
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
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);
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
        let mut lints = BTreeSet::new();
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);
        let expected: Lints = Lints::new(lints);

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
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);
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
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);
        let expected = Lints::new(lints);

        let actual = Lints::try_from_vcs(&mut config).expect("Failed to read lints from VCS");

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn try_from_vcs_enabled_github_id() {
        let mut strings = BTreeMap::new();
        strings.insert("mit.lint.github-id-missing".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        let mut lints = BTreeSet::new();
        lints.insert(DuplicatedTrailers);
        lints.insert(GitHubIdMissing);
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);
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
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);
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
        lints.insert(SubjectNotSeparateFromBody);
        lints.insert(SubjectLongerThan72Characters);
        lints.insert(BodyWiderThan72Characters);
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
    #[error("Failed to read lint config file: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
}
