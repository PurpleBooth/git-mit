use crate::errors::MitCommitMessageLintsError;
use crate::external::vcs::Vcs;
use crate::lints::lib::Lint;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::vec::IntoIter;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Lints {
    lints: BTreeSet<Lint>,
}

impl Lints {
    #[must_use]
    pub fn new(lints: Vec<Lint>) -> Lints {
        Lints {
            lints: BTreeSet::from_iter(lints.into_iter()),
        }
    }

    #[must_use]
    pub fn names(self) -> Vec<&'static str> {
        self.lints.iter().map(|lint| lint.name()).collect()
    }

    #[must_use]
    pub fn config_keys(self) -> Vec<String> {
        self.lints.iter().map(|lint| lint.config_key()).collect()
    }

    /// Create lints from the VCS configuration
    ///
    /// # Errors
    /// If reading from the VCS fails
    pub fn try_from_vcs(config: &mut dyn Vcs) -> Result<Lints, MitCommitMessageLintsError> {
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
    pub fn merge(&self, other: Lints) -> Lints {
        let set_b = BTreeSet::from_iter(other.lints.into_iter());

        Lints::new(self.lints.union(&set_b).cloned().collect::<Vec<_>>())
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
    type Error = MitCommitMessageLintsError;

    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        value
            .into_iter()
            .try_fold(
                vec![],
                |lints: Vec<Lint>, item_name| -> Result<Vec<Lint>, MitCommitMessageLintsError> {
                    match Lint::try_from(item_name) {
                        Err(err) => Err(err),
                        Ok(item) => Ok(vec![lints, vec![item]].concat()),
                    }
                },
            )
            .map(Lints::new)
    }
}

fn get_config_or_default(
    config: &dyn Vcs,
    lint: Lint,
    default: bool,
) -> Result<Option<Lint>, MitCommitMessageLintsError> {
    Ok(config
        .get_bool(&lint.config_key())?
        .or(Some(default))
        .filter(|lint_value| lint_value == &true)
        .map(|_| lint))
}

#[cfg(test)]
mod tests {
    use crate::lints::lib::lints::Lints;

    use crate::lints::Lint::{JiraIssueKeyMissing, PivotalTrackerIdMissing};
    use pretty_assertions::assert_eq;

    use std::collections::BTreeMap;

    use crate::{external::vcs::InMemory, lints::Lint::DuplicatedTrailers};

    use crate::errors::MitCommitMessageLintsError;
    use std::convert::TryInto;

    #[test]
    fn it_returns_an_error_if_one_of_the_names_is_wrong() {
        let lints = vec![
            "pivotal-tracker-id-missing",
            "broken",
            "jira-issue-key-missing",
        ];
        let actual: Result<Lints, MitCommitMessageLintsError> = lints.try_into();

        assert_eq!(true, actual.is_err());
    }

    #[test]
    fn it_can_construct_itself_from_names() {
        let lints = vec!["pivotal-tracker-id-missing", "jira-issue-key-missing"];
        let expected = Ok(Lints::new(vec![
            PivotalTrackerIdMissing,
            JiraIssueKeyMissing,
        ]));
        let actual: Result<Lints, MitCommitMessageLintsError> = lints.try_into();

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_can_give_me_an_into_iterator() {
        let lints = vec![PivotalTrackerIdMissing, JiraIssueKeyMissing];
        let input = Lints::new(lints);

        let expected = vec![PivotalTrackerIdMissing, JiraIssueKeyMissing];
        let actual = input.into_iter().collect::<Vec<_>>();

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_can_give_me_the_names() {
        let lints = vec![PivotalTrackerIdMissing, JiraIssueKeyMissing];
        let input = Lints::new(lints);

        let expected = vec![PivotalTrackerIdMissing.name(), JiraIssueKeyMissing.name()];
        let actual = input.names();

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_can_give_me_the_config_keys() {
        let lints = vec![PivotalTrackerIdMissing, JiraIssueKeyMissing];
        let input = Lints::new(lints);

        let expected = vec![
            PivotalTrackerIdMissing.config_key(),
            JiraIssueKeyMissing.config_key(),
        ];
        let actual = input.config_keys();

        assert_eq!(expected, actual);
    }
    #[test]
    fn defaults() {
        let mut strings = BTreeMap::new();
        let mut config = InMemory::new(&mut strings);

        let actual = Lints::try_from_vcs(&mut config);
        let expected = Ok(Lints::new(vec![DuplicatedTrailers]));

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn disabled_duplicated_trailers() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.duplicated-trailers".into(), "false".into());
        let mut config = InMemory::new(&mut strings);

        let actual = Lints::try_from_vcs(&mut config);
        let expected: Result<Lints, MitCommitMessageLintsError> = Ok(Lints::new(vec![]));

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn enabled_duplicated_trailers() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.duplicated-trailers".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        let actual = Lints::try_from_vcs(&mut config);
        let expected = Ok(Lints::new(vec![DuplicatedTrailers]));

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn enabled_pivotal_tracker_id() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.pivotal-tracker-id-missing".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        let actual = Lints::try_from_vcs(&mut config);
        let expected = Ok(Lints::new(vec![
            DuplicatedTrailers,
            PivotalTrackerIdMissing,
        ]));

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn enabled_jira_issue_key_missing() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.jira-issue-key-missing".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        let actual = Lints::try_from_vcs(&mut config);
        let expected = Ok(Lints::new(vec![DuplicatedTrailers, JiraIssueKeyMissing]));

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn disabled_jira_issue_key_missing() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.jira-issue-key-missing".into(), "false".into());
        let mut config = InMemory::new(&mut strings);

        let actual = Lints::try_from_vcs(&mut config);
        let expected = Ok(Lints::new(vec![DuplicatedTrailers]));

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn two_sets_of_lints_can_be_merged() {
        let set_a = Lints::new(vec![PivotalTrackerIdMissing]);
        let set_b = Lints::new(vec![DuplicatedTrailers]);

        let actual = set_a.merge(set_b);
        let expected = Lints::new(vec![PivotalTrackerIdMissing, DuplicatedTrailers]);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }
    #[test]
    fn when_merging_overlapping_does_not_lead_to_duplication() {
        let set_a = Lints::new(vec![PivotalTrackerIdMissing]);
        let set_b = Lints::new(vec![PivotalTrackerIdMissing, DuplicatedTrailers]);

        let actual = set_a.merge(set_b);
        let expected = Lints::new(vec![DuplicatedTrailers, PivotalTrackerIdMissing]);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }
}
