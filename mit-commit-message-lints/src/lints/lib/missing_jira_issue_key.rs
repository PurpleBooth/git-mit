use regex::Regex;

use crate::lints::lib::problem::Code;
use crate::lints::lib::{CommitMessage, Problem};
use indoc::indoc;

pub(crate) const CONFIG: &str = "jira-issue-key-missing";

const JIRA_HELP_MESSAGE: &str = indoc!(
    "
    Your commit is missing a JIRA Issue Key

    You can fix this by adding a key like `JRA-123` to the commit message
    "
);

const REGEX_JIRA_ISSUE_KEY: &str = r"(?m)(^| )[A-Z]{2,}-[0-9]+( |$)";

fn has_missing_jira_issue_key(commit_message: &CommitMessage) -> bool {
    let re = Regex::new(REGEX_JIRA_ISSUE_KEY).unwrap();
    !commit_message.matches_pattern(&re)
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_missing_jira_issue_key(commit_message) {
        Some(Problem::new(
            JIRA_HELP_MESSAGE.into(),
            Code::JiraIssueKeyMissing,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests_has_missing_jira_issue_key {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn id_present() {
        test_has_missing_jira_issue_key(
            indoc!(
                "
                JRA-123 An example commit

                This is an example commit
                "
            ),
            &None,
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an JRA-123 example commit
                "
            ),
            &None,
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                JRA-123

                This is an example commit
                "
            ),
            &None,
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit

                JRA-123
                "
            ),
            &None,
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit

                JR-123
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_missing() {
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                indoc!(
                    "
                    Your commit is missing a JIRA Issue Key

                    You can fix this by adding a key like `JRA-123` to the commit message
                    "
                )
                .into(),
                Code::JiraIssueKeyMissing,
            )),
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit

                A-123
                "
            ),
            &Some(Problem::new(
                indoc!(
                    "
                    Your commit is missing a JIRA Issue Key

                    You can fix this by adding a key like `JRA-123` to the commit message
                    "
                )
                .into(),
                Code::JiraIssueKeyMissing,
            )),
        );
        test_has_missing_jira_issue_key(
            indoc!(
                "
                An example commit

                This is an example commit

                JRA-
                "
            ),
            &Some(Problem::new(
                indoc!(
                    "
                    Your commit is missing a JIRA Issue Key

                    You can fix this by adding a key like `JRA-123` to the commit message
                    "
                )
                .into(),
                Code::JiraIssueKeyMissing,
            )),
        );
    }

    fn test_has_missing_jira_issue_key(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::new(message.into()));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
