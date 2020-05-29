use std::error::Error;

use crate::{
    external::vcs::Vcs,
    lints::Lints::{DuplicatedTrailers, JiraIssueKeyMissing, PivotalTrackerIdMissing},
};
use regex::Regex;
use std::fmt::Display;

use crate::lints::{
    duplicate_trailers::lint_duplicated_trailers,
    missing_jira_issue_key::lint_missing_jira_issue_key,
    missing_pivotal_tracker_id::lint_missing_pivotal_tracker_id,
};
pub struct CommitMessage<'a> {
    contents: &'a str,
}

impl CommitMessage<'_> {
    #[must_use]
    pub fn new(contents: &str) -> CommitMessage {
        CommitMessage { contents }
    }

    pub fn matches_pattern(&self, re: &Regex) -> bool {
        re.is_match(self.contents)
    }

    #[must_use]
    pub fn get_trailer(&self, trailer: &str) -> Vec<&str> {
        self.contents
            .lines()
            .filter(|line: &&str| CommitMessage::line_has_trailer(trailer, line))
            .collect::<Vec<_>>()
    }

    fn line_has_trailer(trailer: &str, line: &str) -> bool {
        line.starts_with(&format!("{}:", trailer))
    }
}

impl Display for CommitMessage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

#[cfg(test)]
mod test_commit_message {
    use crate::lints::CommitMessage;
    use pretty_assertions::assert_eq;
    use regex::Regex;

    #[test]
    fn with_trailers() {
        let commit = CommitMessage::new(
            r#"Some Commit Message

Anything: Some Trailer
Anything: Some Trailer
Another: Trailer
"#,
        );

        assert_eq!(vec!["Another: Trailer"], commit.get_trailer("Another"));
        assert_eq!(
            vec!["Anything: Some Trailer", "Anything: Some Trailer"],
            commit.get_trailer("Anything")
        )
    }

    #[test]
    fn regex_matching() {
        let commit = CommitMessage::new(
            r#"Some Commit Message

Anything: Some Trailer
Anything: Some Trailer
Another: Trailer
"#,
        );

        assert_eq!(
            true,
            commit.matches_pattern(&Regex::new("[AB]nything:").unwrap())
        );
        assert_eq!(
            false,
            commit.matches_pattern(&Regex::new("N[oO]thing:").unwrap())
        );
    }
}

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
            Lints::DuplicatedTrailers => lint_duplicated_trailers(&format!("{}", commit_message)),
            Lints::PivotalTrackerIdMissing => {
                lint_missing_pivotal_tracker_id(&format!("{}", commit_message))
            },
            Lints::JiraIssueKeyMissing => {
                lint_missing_jira_issue_key(&format!("{}", commit_message))
            },
        }
    }
}

impl std::fmt::Display for Lints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let config_key = match self {
            DuplicatedTrailers => CONFIG_DUPLICATED_TRAILERS,
            PivotalTrackerIdMissing => CONFIG_PIVOTAL_TRACKER_ID_MISSING,
            JiraIssueKeyMissing => CONFIG_JIRA_ISSUE_KEY_MISSING,
        };
        write!(f, "{}", config_key)
    }
}

impl std::convert::TryFrom<&str> for Lints {
    type Error = Box<dyn Error>;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Lints::iterator()
            .zip(Lints::iterator().map(|lint| format!("{}", lint)))
            .filter_map(
                |(lint, name): (Lints, String)| if name == from { Some(lint) } else { None },
            )
            .collect::<Vec<Lints>>()
            .first()
            .copied()
            .ok_or_else(|| -> Box<dyn Error> {
                format!("Could not match {} to a lint", from).into()
            })
    }
}

impl std::convert::From<Lints> for String {
    fn from(from: Lints) -> Self {
        format!("{}", from)
    }
}

pub fn get_lint_configuration(config: &dyn Vcs) -> Vec<Lints> {
    vec![
        config
            .get_bool(&Lints::DuplicatedTrailers.config_key())
            .or(Some(true))
            .filter(bool::clone)
            .map(|_| DuplicatedTrailers),
        config
            .get_bool(&Lints::PivotalTrackerIdMissing.config_key())
            .filter(bool::clone)
            .map(|_| PivotalTrackerIdMissing),
        config
            .get_bool(&Lints::JiraIssueKeyMissing.config_key())
            .filter(bool::clone)
            .map(|_| JiraIssueKeyMissing),
    ]
    .into_iter()
    .flatten()
    .collect()
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
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use crate::{
        external::vcs::InMemory,
        lints::{
            get_lint_configuration,
            Lints,
            Lints::{DuplicatedTrailers, JiraIssueKeyMissing, PivotalTrackerIdMissing},
        },
    };

    #[test]
    fn defaults() {
        let mut strings = HashMap::new();
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected = vec![DuplicatedTrailers];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn disabled_duplicated_trailers() {
        let mut strings = HashMap::new();
        strings.insert("pb.lint.duplicated-trailers".into(), "false".into());
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Vec<Lints> = vec![];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn enabled_duplicated_trailers() {
        let mut strings = HashMap::new();
        strings.insert("pb.lint.duplicated-trailers".into(), "true".into());
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Vec<Lints> = vec![DuplicatedTrailers];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn enabled_pivotal_tracker_id() {
        let mut strings = HashMap::new();
        strings.insert("pb.lint.pivotal-tracker-id-missing".into(), "true".into());
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Vec<Lints> = vec![DuplicatedTrailers, PivotalTrackerIdMissing];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn enabled_jira_issue_key_missing() {
        let mut strings = HashMap::new();
        strings.insert("pb.lint.jira-issue-key-missing".into(), "true".into());
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Vec<Lints> = vec![DuplicatedTrailers, JiraIssueKeyMissing];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn disabled_jira_issue_key_missing() {
        let mut strings = HashMap::new();
        strings.insert("pb.lint.jira-issue-key-missing".into(), "false".into());
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Vec<Lints> = vec![DuplicatedTrailers];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }
}

#[cfg(test)]
mod tests_can_enable_lints_via_a_command {
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use crate::{
        external::vcs::InMemory,
        lints::{set_lint_status, Lints::PivotalTrackerIdMissing},
    };

    #[test]
    fn we_can_enable_lints() {
        let mut strings = HashMap::new();
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
        let mut strings = HashMap::new();
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
) -> Result<(), Box<dyn Error>> {
    lints
        .iter()
        .try_for_each(|lint| vcs.set_str(&lint.config_key(), &status.to_string()))
}

#[must_use]
pub fn lint(commit_message: &CommitMessage, lints: Vec<Lints>) -> Vec<LintProblem> {
    lints
        .into_iter()
        .flat_map(|lint| match lint {
            Lints::DuplicatedTrailers => lint_duplicated_trailers(&format!("{}", commit_message)),
            Lints::PivotalTrackerIdMissing => {
                lint_missing_pivotal_tracker_id(&format!("{}", commit_message))
            },
            Lints::JiraIssueKeyMissing => {
                lint_missing_jira_issue_key(&format!("{}", commit_message))
            },
        })
        .collect::<Vec<LintProblem>>()
}

#[derive(Debug)]
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
