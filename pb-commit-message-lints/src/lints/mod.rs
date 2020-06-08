use std::fmt::Display;

use crate::{
    errors::PbCommitMessageLintsError,
    external::vcs::Vcs,
    lints::{
        duplicate_trailers::lint_duplicated_trailers,
        missing_jira_issue_key::lint_missing_jira_issue_key,
        missing_pivotal_tracker_id::lint_missing_pivotal_tracker_id,
        Lints::{DuplicatedTrailers, JiraIssueKeyMissing, PivotalTrackerIdMissing},
    },
};

pub mod lib;

use lib::CommitMessage;

/// The lints that are supported
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Lints {
    DuplicatedTrailers,
    PivotalTrackerIdMissing,
    JiraIssueKeyMissing,
}

const CONFIG_DUPLICATED_TRAILERS: &str = "duplicated-trailers";
const CONFIG_PIVOTAL_TRACKER_ID_MISSING: &str = "pivotal-tracker-id-missing";
const CONFIG_JIRA_ISSUE_KEY_MISSING: &str = "jira-issue-key-missing";

impl Lints {
    pub fn iterator() -> impl Iterator<Item = Lints> {
        static LINTS: [Lints; 3] = [
            DuplicatedTrailers,
            PivotalTrackerIdMissing,
            JiraIssueKeyMissing,
        ];
        LINTS.iter().copied()
    }

    #[must_use]
    pub fn config_key(self) -> String {
        format!("pb.lint.{}", self)
    }

    #[must_use]
    pub fn lint(self, commit_message: &CommitMessage) -> Option<LintProblem> {
        match self {
            Lints::DuplicatedTrailers => lint_duplicated_trailers(commit_message),
            Lints::PivotalTrackerIdMissing => lint_missing_pivotal_tracker_id(commit_message),
            Lints::JiraIssueKeyMissing => lint_missing_jira_issue_key(commit_message),
        }
    }
}

impl std::fmt::Display for Lints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Into<&str> for Lints {
    fn into(self) -> &'static str {
        self.name()
    }
}

impl std::convert::TryFrom<&str> for Lints {
    type Error = PbCommitMessageLintsError;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Lints::iterator()
            .zip(Lints::iterator().map(|lint| format!("{}", lint)))
            .filter_map(
                |(lint, name): (Lints, String)| if name == from { Some(lint) } else { None },
            )
            .collect::<Vec<Lints>>()
            .first()
            .copied()
            .ok_or_else(|| PbCommitMessageLintsError::LintNotFoundError(from.into()))
    }
}

impl std::convert::From<Lints> for String {
    fn from(from: Lints) -> Self {
        format!("{}", from)
    }
}

/// Get the lints that are currently enabled
///
/// # Errors
///
/// If there's an error reading from the configuration source
pub fn get_lint_configuration(config: &dyn Vcs) -> Result<Vec<Lints>, PbCommitMessageLintsError> {
    Ok(vec![
        get_config_or_default(config, Lints::DuplicatedTrailers, true)?,
        get_config_or_default(config, Lints::PivotalTrackerIdMissing, false)?,
        get_config_or_default(config, Lints::JiraIssueKeyMissing, false)?,
    ]
    .into_iter()
    .flatten()
    .collect())
}

fn get_config_or_default(
    config: &dyn Vcs,
    lint: Lints,
    default: bool,
) -> Result<Option<Lints>, PbCommitMessageLintsError> {
    Ok(config
        .get_bool(&lint.config_key())?
        .or(Some(default))
        .filter(|lint_value| lint_value == &true)
        .map(|_| lint))
}

#[cfg(test)]
mod tests_lints {
    use std::convert::TryInto;

    use pretty_assertions::assert_eq;

    use crate::lints::{Lints, Lints::PivotalTrackerIdMissing};

    #[test]
    fn it_is_convertible_to_string() {
        let string: String = Lints::PivotalTrackerIdMissing.into();
        assert_eq!("pivotal-tracker-id-missing".to_string(), string)
    }

    #[test]
    fn it_can_be_created_from_string() {
        let lint: Lints = "pivotal-tracker-id-missing".try_into().unwrap();
        assert_eq!(PivotalTrackerIdMissing, lint)
    }

    #[test]
    fn it_is_printable() {
        assert_eq!(
            "pivotal-tracker-id-missing",
            &format!("{}", Lints::PivotalTrackerIdMissing)
        )
    }
}

mod missing_pivotal_tracker_id;

mod duplicate_trailers;

mod missing_jira_issue_key;

#[cfg(test)]
mod tests_get_lint_configuration {
    use std::collections::BTreeMap;

    use pretty_assertions::assert_eq;

    use crate::{
        errors::PbCommitMessageLintsError,
        external::vcs::InMemory,
        lints::{
            get_lint_configuration,
            Lints,
            Lints::{DuplicatedTrailers, JiraIssueKeyMissing, PivotalTrackerIdMissing},
        },
    };

    #[test]
    fn defaults() {
        let mut strings = BTreeMap::new();
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected = Ok(vec![DuplicatedTrailers]);

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
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Result<Vec<Lints>, PbCommitMessageLintsError> = Ok(vec![]);

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
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Result<Vec<Lints>, PbCommitMessageLintsError> = Ok(vec![DuplicatedTrailers]);

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
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Result<Vec<Lints>, PbCommitMessageLintsError> =
            Ok(vec![DuplicatedTrailers, PivotalTrackerIdMissing]);

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
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Result<Vec<Lints>, PbCommitMessageLintsError> =
            Ok(vec![DuplicatedTrailers, JiraIssueKeyMissing]);

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
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected = Ok(vec![DuplicatedTrailers]);

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }
}

#[cfg(test)]
mod tests_can_enable_lints_via_a_command {
    use std::collections::BTreeMap;

    use pretty_assertions::assert_eq;

    use crate::{
        external::vcs::InMemory,
        lints::{set_lint_status, Lints::PivotalTrackerIdMissing},
    };

    #[test]
    fn we_can_enable_lints() {
        let mut strings = BTreeMap::new();
        strings.insert("pb.lint.pivotal-tracker-id-missing".into(), "false".into());
        let mut config = InMemory::new(&mut strings);

        set_lint_status(&[PivotalTrackerIdMissing], &mut config, true).unwrap();

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

        set_lint_status(&[PivotalTrackerIdMissing], &mut config, false).unwrap();

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
    lints: &[Lints],
    vcs: &mut dyn Vcs,
    status: bool,
) -> Result<(), PbCommitMessageLintsError> {
    lints
        .iter()
        .try_for_each(|lint| vcs.set_str(&lint.config_key(), &status.to_string()))?;
    Ok(())
}

#[must_use]
pub fn lint(commit_message: &CommitMessage, lints: Vec<Lints>) -> Vec<LintProblem> {
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

impl Lints {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            DuplicatedTrailers => CONFIG_DUPLICATED_TRAILERS,
            PivotalTrackerIdMissing => CONFIG_PIVOTAL_TRACKER_ID_MISSING,
            JiraIssueKeyMissing => CONFIG_JIRA_ISSUE_KEY_MISSING,
        }
    }
}
