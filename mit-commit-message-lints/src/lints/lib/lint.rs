use crate::errors::MitCommitMessageLintsError;
use crate::lints::lib::duplicate_trailers::lint_duplicated_trailers;
use crate::lints::lib::lint::Lint::{
    DuplicatedTrailers, JiraIssueKeyMissing, PivotalTrackerIdMissing,
};
use crate::lints::lib::missing_jira_issue_key::lint_missing_jira_issue_key;
use crate::lints::lib::missing_pivotal_tracker_id::lint_missing_pivotal_tracker_id;
use crate::lints::lib::problem::Problem;
use crate::lints::lib::{
    duplicate_trailers, missing_jira_issue_key, missing_pivotal_tracker_id, CommitMessage, Lints,
};
use std::convert::TryInto;

/// The lints that are supported
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum Lint {
    DuplicatedTrailers,
    PivotalTrackerIdMissing,
    JiraIssueKeyMissing,
}

const CONFIG_KEY_PREFIX: &str = "pb.lint";

impl std::convert::TryFrom<&str> for Lint {
    type Error = MitCommitMessageLintsError;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Lint::iterator()
            .zip(Lint::iterator().map(|lint| format!("{}", lint)))
            .filter_map(|(lint, name): (Lint, String)| if name == from { Some(lint) } else { None })
            .collect::<Vec<Lint>>()
            .first()
            .copied()
            .ok_or_else(|| MitCommitMessageLintsError::LintNotFoundError(from.into()))
    }
}

impl std::convert::From<Lint> for String {
    fn from(from: Lint) -> Self {
        format!("{}", from)
    }
}

impl Into<&str> for Lint {
    fn into(self) -> &'static str {
        self.name()
    }
}

impl Lint {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            DuplicatedTrailers => duplicate_trailers::CONFIG_DUPLICATED_TRAILERS,
            PivotalTrackerIdMissing => {
                missing_pivotal_tracker_id::CONFIG_PIVOTAL_TRACKER_ID_MISSING
            }
            JiraIssueKeyMissing => missing_jira_issue_key::CONFIG_JIRA_ISSUE_KEY_MISSING,
        }
    }
}

impl Lint {
    pub fn iterator() -> impl Iterator<Item = Lint> {
        static LINTS: [Lint; 3] = [
            DuplicatedTrailers,
            PivotalTrackerIdMissing,
            JiraIssueKeyMissing,
        ];
        LINTS.iter().copied()
    }

    #[must_use]
    pub fn config_key(self) -> String {
        format!("{}.{}", CONFIG_KEY_PREFIX, self)
    }

    #[must_use]
    pub fn lint(self, commit_message: &CommitMessage) -> Option<Problem> {
        match self {
            Lint::DuplicatedTrailers => lint_duplicated_trailers(commit_message),
            Lint::PivotalTrackerIdMissing => lint_missing_pivotal_tracker_id(commit_message),
            Lint::JiraIssueKeyMissing => lint_missing_jira_issue_key(commit_message),
        }
    }

    /// Try and convert a list of names into lints
    ///
    /// # Errors
    /// If the lint does not exist
    pub fn from_names(names: Vec<&str>) -> Result<Vec<Lint>, MitCommitMessageLintsError> {
        let lints: Lints = names.try_into()?;
        Ok(lints.into_iter().collect())
    }
}

#[cfg(test)]
mod tests_lints {
    use std::convert::TryInto;

    use pretty_assertions::assert_eq;

    use crate::lints::Lint;

    #[test]
    fn it_is_convertible_to_string() {
        let string: String = Lint::PivotalTrackerIdMissing.into();
        assert_eq!("pivotal-tracker-id-missing".to_string(), string)
    }

    #[test]
    fn it_can_be_created_from_string() {
        let lint: Lint = "pivotal-tracker-id-missing".try_into().unwrap();
        assert_eq!(Lint::PivotalTrackerIdMissing, lint)
    }

    #[test]
    fn it_is_printable() {
        assert_eq!(
            "pivotal-tracker-id-missing",
            &format!("{}", Lint::PivotalTrackerIdMissing)
        )
    }
}

impl std::fmt::Display for Lint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
