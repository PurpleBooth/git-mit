use std::fmt::Display;

use crate::{
    errors::PbCommitMessageLintsError,
    external::vcs::Vcs,
    lints::{
        duplicate_trailers::lint_duplicated_trailers,
        missing_jira_issue_key::lint_missing_jira_issue_key,
        missing_pivotal_tracker_id::lint_missing_pivotal_tracker_id,
        Lint::{DuplicatedTrailers, JiraIssueKeyMissing, PivotalTrackerIdMissing},
    },
};

pub mod lib;

use lib::CommitMessage;
use lib::Lints;
use std::convert::TryInto;

/// The lints that are supported
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Lint {
    DuplicatedTrailers,
    PivotalTrackerIdMissing,
    JiraIssueKeyMissing,
}

const CONFIG_DUPLICATED_TRAILERS: &str = "duplicated-trailers";
const CONFIG_PIVOTAL_TRACKER_ID_MISSING: &str = "pivotal-tracker-id-missing";
const CONFIG_JIRA_ISSUE_KEY_MISSING: &str = "jira-issue-key-missing";

const CONFIG_KEY_PREFIX: &str = "pb.lint";

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
    pub fn lint(self, commit_message: &CommitMessage) -> Option<LintProblem> {
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
    pub fn from_names(names: Vec<&str>) -> Result<Vec<Lint>, PbCommitMessageLintsError> {
        let lints: Lints = names.try_into()?;
        Ok(lints.into_iter().collect())
    }
}

impl std::fmt::Display for Lint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Into<&str> for Lint {
    fn into(self) -> &'static str {
        self.name()
    }
}

impl std::convert::TryFrom<&str> for Lint {
    type Error = PbCommitMessageLintsError;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Lint::iterator()
            .zip(Lint::iterator().map(|lint| format!("{}", lint)))
            .filter_map(|(lint, name): (Lint, String)| if name == from { Some(lint) } else { None })
            .collect::<Vec<Lint>>()
            .first()
            .copied()
            .ok_or_else(|| PbCommitMessageLintsError::LintNotFoundError(from.into()))
    }
}

impl std::convert::From<Lint> for String {
    fn from(from: Lint) -> Self {
        format!("{}", from)
    }
}

#[cfg(test)]
mod tests_lints {
    use std::convert::TryInto;

    use pretty_assertions::assert_eq;

    use crate::lints::{Lint, Lint::PivotalTrackerIdMissing};

    #[test]
    fn it_is_convertible_to_string() {
        let string: String = Lint::PivotalTrackerIdMissing.into();
        assert_eq!("pivotal-tracker-id-missing".to_string(), string)
    }

    #[test]
    fn it_can_be_created_from_string() {
        let lint: Lint = "pivotal-tracker-id-missing".try_into().unwrap();
        assert_eq!(PivotalTrackerIdMissing, lint)
    }

    #[test]
    fn it_is_printable() {
        assert_eq!(
            "pivotal-tracker-id-missing",
            &format!("{}", Lint::PivotalTrackerIdMissing)
        )
    }
}

mod missing_pivotal_tracker_id;

mod duplicate_trailers;

mod missing_jira_issue_key;

#[cfg(test)]
mod tests_can_enable_lints_via_a_command {
    use std::collections::BTreeMap;

    use pretty_assertions::assert_eq;

    use crate::lints::lib::Lints;
    use crate::{
        external::vcs::InMemory,
        lints::{set_lint_status, Lint::PivotalTrackerIdMissing},
    };

    #[test]
    fn we_can_enable_lints() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.pivotal-tracker-id-missing".into(), "false".into());
        let mut config = InMemory::new(&mut strings);

        set_lint_status(Lints::new(vec![PivotalTrackerIdMissing]), &mut config, true).unwrap();

        let expected = "true".to_string();
        let actual = strings
            .get("pb.lint.pivotal-tracker-id-missing")
            .unwrap()
            .clone();
        assert_eq!(expected, actual);
    }

    #[test]
    fn we_can_disable_lints() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.pivotal-tracker-id-missing".into(), "true".into());
        let mut config = InMemory::new(&mut strings);

        set_lint_status(
            Lints::new(vec![PivotalTrackerIdMissing]),
            &mut config,
            false,
        )
        .unwrap();

        let expected = "false".to_string();
        let actual = strings
            .get("pb.lint.pivotal-tracker-id-missing")
            .unwrap()
            .clone();
        assert_eq!(expected, actual);
    }
}

/// # Errors
///
/// Errors if writing to the VCS config fails
pub fn set_lint_status(
    lints: Lints,
    vcs: &mut dyn Vcs,
    status: bool,
) -> Result<(), PbCommitMessageLintsError> {
    lints
        .config_keys()
        .into_iter()
        .try_for_each(|lint| vcs.set_str(&lint, &status.to_string()))?;
    Ok(())
}

#[must_use]
pub fn lint(commit_message: &CommitMessage, lints: Lints) -> Vec<LintProblem> {
    lints
        .into_iter()
        .flat_map(|lint| lint.lint(commit_message))
        .collect::<Vec<LintProblem>>()
}

#[derive(Debug, Eq, PartialEq)]
pub struct LintProblem {
    help: String,
    code: LintCode,
}

impl LintProblem {
    #[must_use]
    pub fn new(help: String, code: LintCode) -> LintProblem {
        LintProblem { help, code }
    }

    #[must_use]
    pub fn code(self) -> LintCode {
        self.code
    }
}

impl Display for LintProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.help)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(i32)]
pub enum LintCode {
    DuplicatedTrailers = 3,
    PivotalTrackerIdMissing,
    JiraIssueKeyMissing,
}

impl Lint {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            DuplicatedTrailers => CONFIG_DUPLICATED_TRAILERS,
            PivotalTrackerIdMissing => CONFIG_PIVOTAL_TRACKER_ID_MISSING,
            JiraIssueKeyMissing => CONFIG_JIRA_ISSUE_KEY_MISSING,
        }
    }
}
