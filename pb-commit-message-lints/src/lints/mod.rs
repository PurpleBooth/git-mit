use std::{collections::HashSet, error::Error, iter::FromIterator};

use enum_iterator::IntoEnumIterator;
use regex::Regex;

use crate::{
    external::vcs::Vcs,
    lints::Lints::{DuplicatedTrailers, JiraIssueKeyMissing, PivotalTrackerIdMissing},
};

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 2] = ["Signed-off-by", "Co-authored-by"];
const REGEX_PIVOTAL_TRACKER_ID: &str =
    r"(?i)\[(((finish|fix)(ed|es)?|complete[ds]?|deliver(s|ed)?) )?#\d+([, ]#\d+)*]";
const REGEX_JIRA_ISSUE_KEY: &str = r"(?m)(^| )[A-Z]{2,}-[0-9]+( |$)";

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
#[derive(Debug, Eq, PartialEq, IntoEnumIterator, Copy, Clone)]
pub enum Lints {
    DuplicatedTrailers,
    PivotalTrackerIdMissing,
    JiraIssueKeyMissing,
}

const CONFIG_DUPLICATED_TRAILERS: &str = "duplicated-trailers";
const CONFIG_PIVOTAL_TRACKER_ID_MISSING: &str = "pivotal-tracker-id-missing";
const CONFIG_JIRA_ISSUE_KEY_MISSING: &str = "jira-issue-key-missing";

impl std::fmt::Display for Lints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", to_static_string(*self))
    }
}

impl std::convert::TryFrom<&str> for Lints {
    type Error = Box<dyn Error>;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Lints::into_enum_iter()
            .zip(Lints::into_enum_iter().map(|lint| lint.into()))
            .filter_map(|(lint, name): (Lints, &str)| if name == from { Some(lint) } else { None })
            .collect::<Vec<Lints>>()
            .first()
            .copied()
            .ok_or_else(|| -> Box<dyn Error> {
                format!("Could not match {} to a lint", from).into()
            })
    }
}

impl std::convert::From<Lints> for &'static str {
    fn from(from: Lints) -> Self {
        to_static_string(from)
    }
}

impl std::convert::From<Lints> for String {
    fn from(from: Lints) -> Self {
        String::from(to_static_string(from))
    }
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

/// Look at a git config and work out what lints should be turned on and off
///
/// # Example
///
/// ```
/// use git2::Repository;
/// use pb_commit_message_lints::{
///     external::vcs::Git2,
///     lints::{get_lint_configuration, Lints::DuplicatedTrailers},
/// };
/// use tempfile::TempDir;
/// let config = TempDir::new()
///     .map(TempDir::into_path)
///     .map(|path| path.join("repository"))
///     .map(Repository::init)
///     .expect("Failed to initialise the repository")
///     .expect("Failed create temporary directory")
///     .config()
///     .map(Git2::new)
///     .expect("Failed to get configuration");
///
/// assert_eq!(get_lint_configuration(&config), vec![DuplicatedTrailers],)
/// ```
///
/// # Errors
///
/// Will return `Err` if we can't read the git configuration for some reason or
/// it's not parsable
pub fn get_lint_configuration(config: &dyn Vcs) -> Vec<Lints> {
    vec![
        config
            .get_bool(&format!("pb.lint.{}", Lints::DuplicatedTrailers))
            .or(Some(true))
            .filter(bool::clone)
            .map(|_| DuplicatedTrailers),
        config
            .get_bool(&format!("pb.lint.{}", Lints::PivotalTrackerIdMissing))
            .filter(bool::clone)
            .map(|_| PivotalTrackerIdMissing),
        config
            .get_bool(&format!("pb.lint.{}", Lints::JiraIssueKeyMissing))
            .filter(bool::clone)
            .map(|_| JiraIssueKeyMissing),
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// Check if a commit message message has duplicated trailers with names in
///
/// # Example
///
/// ```
/// use pb_commit_message_lints::lints::{has_duplicated_trailers, CommitMessage};
/// assert_eq!(
///     has_duplicated_trailers(&CommitMessage::new(
///         r#"
/// An example commit
///
/// This is an example commit without any duplicate trailers
///
/// Signed-off-by: Billie Thompson <email@example.com>
/// Signed-off-by: Billie Thompson <email@example.com>
/// "#
///     )),
///     vec![String::from("Signed-off-by")]
/// );
///
/// assert_eq!(
///     has_duplicated_trailers(&CommitMessage::new(
///         r#"
/// An example commit
///
/// This is an example commit without any duplicate trailers
///
/// Co-authored-by: Billie Thompson <email@example.com>
/// Co-authored-by: Billie Thompson <email@example.com>
/// "#
///     )),
///     vec![String::from("Co-authored-by")]
/// );
/// ```
#[must_use]
pub fn has_duplicated_trailers(commit_message: &CommitMessage) -> Vec<String> {
    TRAILERS_TO_CHECK_FOR_DUPLICATES
        .iter()
        .filter_map(|trailer| filter_without_duplicates(commit_message, trailer))
        .collect::<Vec<String>>()
}

#[must_use]
pub fn has_missing_jira_issue_key(commit_message: &CommitMessage) -> bool {
    let re = Regex::new(REGEX_JIRA_ISSUE_KEY).unwrap();
    !commit_message.matches_pattern(&re)
}

fn filter_without_duplicates(commit_message: &CommitMessage, trailer: &str) -> Option<String> {
    Some(trailer)
        .map(String::from)
        .filter(|trailer| has_duplicated_trailer(commit_message, trailer))
}

/// Check if a commit message message has a pivotal tracker id in it
///
/// # Example
///
/// ```
/// use pb_commit_message_lints::lints::{has_missing_pivotal_tracker_id, CommitMessage};
/// assert_eq!(
///     has_missing_pivotal_tracker_id(&CommitMessage::new(
///         r#"
/// [fix #12345678] correct bug the build
/// "#,
///     )),
///     false
/// );
/// assert_eq!(
///     has_missing_pivotal_tracker_id(&CommitMessage::new(
///         r#"
/// Add a new commit linter
///
/// This will add a new linter. This linter checks for the presence of a Pivotal Tracker Id. In this
/// example I have forgotten my Id.
/// "#,
///     )),
///     true
/// );
///
/// assert_eq!(
///     has_missing_pivotal_tracker_id(&CommitMessage::new(
///         r#"
/// Add a new commit linter
///
/// This will add a new linter. This linter checks for the presence of a Pivotal Tracker Id. In this
/// example I have remembered my Id
///
/// [deliver #12345678,#88335556,#87654321]
/// "#
///     )),
///     false
/// );
/// ```
#[must_use]
pub fn has_missing_pivotal_tracker_id(commit_message: &CommitMessage) -> bool {
    has_no_pivotal_tracker_id(commit_message)
}

fn has_no_pivotal_tracker_id(text: &CommitMessage) -> bool {
    let re = Regex::new(REGEX_PIVOTAL_TRACKER_ID).unwrap();
    !text.matches_pattern(&re)
}

fn has_duplicated_trailer(commit_message: &CommitMessage, trailer: &str) -> bool {
    Some(commit_message.get_trailer(trailer))
        .map(|trailers| (trailers.clone(), trailers.clone()))
        .map(|(commit, unique)| (commit, HashSet::<&str>::from_iter(unique)))
        .map(|(commit, unique)| commit.len() != unique.len())
        .unwrap()
}

#[cfg(test)]
mod tests_has_duplicated_trailers {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn duplicated_trailers() {
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers
"#,
            &[],
        );
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#,
            &["Signed-off-by".into(), "Co-authored-by".into()],
        );
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#,
            &["Signed-off-by".into()],
        );
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#,
            &["Co-authored-by".into()],
        );
    }

    fn test_has_duplicated_trailers(message: &str, expected: &[String]) {
        let actual = has_duplicated_trailers(&CommitMessage::new(message));
        assert_eq!(
            actual, expected,
            "Expected {:?}, found {:?}",
            expected, actual
        );
    }
}

#[cfg(test)]
mod tests_has_duplicated_trailer {
    use crate::lints::{has_duplicated_trailer, CommitMessage};

    fn test_has_duplicated_trailer(message: &str, trailer: &str, expected: bool) {
        let actual = has_duplicated_trailer(&CommitMessage::new(message), trailer);
        assert_eq!(
            actual, expected,
            "Message {:?} with trailer {:?} should have returned {:?}, found {:?}",
            message, trailer, expected, actual
        );
    }

    #[test]
    fn no_trailer() {
        test_has_duplicated_trailer(
            r#"
An example commit

This is an example commit without any duplicate trailers
"#,
            "Signed-off-by",
            false,
        );
    }

    #[test]
    fn duplicated_trailer() {
        test_has_duplicated_trailer(
            r#"
An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#,
            "Signed-off-by",
            true,
        );
    }

    #[test]
    fn two_trailers_but_no_duplicates() {
        test_has_duplicated_trailer(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <billie@example.com>
Signed-off-by: Ada Lovelace <ada@example.com>
"#,
            "Signed-off-by",
            false,
        );
    }

    #[test]
    fn one_trailer() {
        test_has_duplicated_trailer(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
"#,
            "Signed-off-by",
            false,
        );
    }

    #[test]
    fn missing_colon_in_trailer() {
        test_has_duplicated_trailer(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by Billie Thompson <email@example.com>
Signed-off-by Billie Thompson <email@example.com>
"#,
            "Signed-off-by",
            false,
        );
    }

    #[test]
    fn customised_trailer() {
        test_has_duplicated_trailer(
            r#"
An example commit

This is an example commit with duplicate trailers

Anything: Billie Thompson <email@example.com>
Anything: Billie Thompson <email@example.com>
"#,
            "Anything",
            true,
        );
    }
}

#[cfg(test)]
mod tests_has_missing_pivotal_tracker_id {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn with_id() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678]
    "#,
            false,
        );
    }

    fn test_has_missing_pivotal_tracker_id(message: &str, expected: bool) {
        let actual = has_missing_pivotal_tracker_id(&CommitMessage::new(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }

    #[test]
    fn multiple_ids() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678,#87654321]
    "#,
            false,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678,#87654321,#11223344]
    "#,
            false,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678 #87654321 #11223344]
    "#,
            false,
        );
    }

    #[test]
    fn id_with_fixed_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fix #12345678]
    "#,
            false,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[FIX #12345678]
    "#,
            false,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[Fix #12345678]
    "#,
            false,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fixed #12345678]
    "#,
            false,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fixes #12345678]
    "#,
            false,
        );
    }

    #[test]
    fn id_with_complete_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[complete #12345678]
    "#,
            false,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[completed #12345678]
    "#,
            false,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[Completed #12345678]
    "#,
            false,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[completes #12345678]
    "#,
            false,
        );
    }

    #[test]
    fn id_with_finished_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finish #12345678]
    "#,
            false,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finished #12345678]
    "#,
            false,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finishes #12345678]
    "#,
            false,
        );
    }

    #[test]
    fn id_with_delivered_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[deliver #12345678]
    "#,
            false,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[delivered #12345678]
    "#,
            false,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[delivers #12345678]
    "#,
            false,
        );
    }

    #[test]
    fn id_with_state_change_and_multiple_ids() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fix #12345678 #12345678]
    "#,
            false,
        );
    }

    #[test]
    fn id_with_prefixed_text() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

Finally [fix #12345678 #12345678]
    "#,
            false,
        );
    }

    #[test]
    fn invalid_state_change() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fake #12345678]
    "#,
            true,
        );
    }

    #[test]
    fn missing_id_with_square_brackets() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit
    "#,
            true,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#]
    "#,
            true,
        );
    }
}

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

fn to_static_string(lint: Lints) -> &'static str {
    match lint {
        Lints::DuplicatedTrailers => CONFIG_DUPLICATED_TRAILERS,
        Lints::PivotalTrackerIdMissing => CONFIG_PIVOTAL_TRACKER_ID_MISSING,
        Lints::JiraIssueKeyMissing => CONFIG_JIRA_ISSUE_KEY_MISSING,
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
        .try_for_each(|lint| vcs.set_str(&format!("pb.lint.{}", lint), &status.to_string()))
}

#[cfg(test)]
mod tests_has_missing_jira_issue_key {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn id_present() {
        test_has_missing_jira_issue_key(
            r#"JRA-123 An example commit

This is an example commit
"#,
            false,
        );
        test_has_missing_jira_issue_key(
            r#"An example commit

This is an JRA-123 example commit
"#,
            false,
        );
        test_has_missing_jira_issue_key(
            r#"An example commit

JRA-123

This is an example commit
"#,
            false,
        );
        test_has_missing_jira_issue_key(
            r#"
An example commit

This is an example commit

JRA-123
    "#,
            false,
        );
        test_has_missing_jira_issue_key(
            r#"
An example commit

This is an example commit

JR-123
    "#,
            false,
        );
    }

    #[test]
    fn id_missing() {
        test_has_missing_jira_issue_key(
            r#"
An example commit

This is an example commit
    "#,
            true,
        );
        test_has_missing_jira_issue_key(
            r#"
An example commit

This is an example commit

A-123
    "#,
            true,
        );
        test_has_missing_jira_issue_key(
            r#"
An example commit

This is an example commit

JRA-
    "#,
            true,
        );
    }

    fn test_has_missing_jira_issue_key(message: &str, expected: bool) {
        let actual = has_missing_jira_issue_key(&CommitMessage::new(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
