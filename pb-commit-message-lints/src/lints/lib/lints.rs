use crate::lints::Lint;

use crate::errors::PbCommitMessageLintsError;
use std::convert::{TryFrom, TryInto};
use std::vec::IntoIter;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Lints {
    lints: Vec<Lint>,
}

impl Lints {
    #[must_use]
    pub fn new(lints: Vec<Lint>) -> Lints {
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
}

impl std::iter::IntoIterator for Lints {
    type Item = Lint;
    type IntoIter = IntoIter<Lint>;

    fn into_iter(self) -> Self::IntoIter {
        self.lints.into_iter()
    }
}

impl TryInto<Lints> for Vec<&str> {
    type Error = PbCommitMessageLintsError;

    fn try_into(self) -> Result<Lints, Self::Error> {
        self.into_iter()
            .try_fold(
                vec![],
                |lints: Vec<Lint>, item_name| -> Result<Vec<Lint>, PbCommitMessageLintsError> {
                    match Lint::try_from(item_name) {
                        Err(err) => Err(err),
                        Ok(item) => Ok(vec![lints, vec![item]].concat()),
                    }
                },
            )
            .map(Lints::new)
    }
}

#[cfg(test)]
mod tests {
    use crate::lints::lib::lints::Lints;

    use crate::lints::Lint::{JiraIssueKeyMissing, PivotalTrackerIdMissing};
    use pretty_assertions::assert_eq;

    use crate::errors::PbCommitMessageLintsError;
    use std::convert::TryInto;

    #[test]
    fn it_returns_an_error_if_one_of_the_names_is_wrong() {
        let lints = vec![
            "pivotal-tracker-id-missing",
            "broken",
            "jira-issue-key-missing",
        ];
        let actual: Result<Lints, PbCommitMessageLintsError> = lints.try_into();

        assert_eq!(true, actual.is_err());
    }

    #[test]
    fn it_can_construct_itself_from_names() {
        let lints = vec!["pivotal-tracker-id-missing", "jira-issue-key-missing"];
        let expected = Ok(Lints::new(vec![
            PivotalTrackerIdMissing,
            JiraIssueKeyMissing,
        ]));
        let actual: Result<Lints, PbCommitMessageLintsError> = lints.try_into();

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
}
