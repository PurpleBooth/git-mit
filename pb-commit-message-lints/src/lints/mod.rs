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

use crate::lints::collection::Lints;
use lib::CommitMessage;
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

/// Get the lints that are currently enabled
///
/// # Errors
///
/// If there's an error reading from the configuration source
pub fn get_lint_configuration(config: &dyn Vcs) -> Result<Lints, PbCommitMessageLintsError> {
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

fn get_config_or_default(
    config: &dyn Vcs,
    lint: Lint,
    default: bool,
) -> Result<Option<Lint>, PbCommitMessageLintsError> {
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
mod tests_get_lint_configuration {
    use std::collections::BTreeMap;

    use pretty_assertions::assert_eq;

    use crate::lints::collection::Lints;
    use crate::{
        errors::PbCommitMessageLintsError,
        external::vcs::InMemory,
        lints::{
            get_lint_configuration,
            Lint::{DuplicatedTrailers, JiraIssueKeyMissing, PivotalTrackerIdMissing},
        },
    };

    #[test]
    fn defaults() {
        let mut strings = BTreeMap::new();
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
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
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected: Result<Lints, PbCommitMessageLintsError> = Ok(Lints::new(vec![]));

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
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
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
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
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
        let config = InMemory::new(&mut strings);

        let actual = get_lint_configuration(&config);
        let expected = Ok(Lints::new(vec![DuplicatedTrailers]));

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

    use crate::lints::collection::Lints;
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

pub mod collection {
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
        use crate::lints::collection::Lints;

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
}
