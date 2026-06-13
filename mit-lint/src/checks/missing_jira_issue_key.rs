use std::sync::LazyLock;

use mit_commit::CommitMessage;
use regex::Regex;

use crate::model::{Code, Problem, ProblemBuilder};

/// Canonical lint ID
pub const CONFIG: &str = "jira-issue-key-missing";
/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to add the issue key because it allows us to link code back to the motivations \
for doing it, and in some cases provide an audit trail for compliance purposes.

You can fix this by adding a key like `JRA-123` to the commit message" ;
/// Description of the problem
pub const ERROR: &str = "Your commit message is missing a JIRA Issue Key";

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?mi)\b[A-Z]{2,}-\d+\b").unwrap());

pub struct JiraIssueKeyConfig;
impl Default for JiraIssueKeyConfig {
    fn default() -> Self {
        Self
    }
}

/// Checks if the commit message contains a JIRA issue key
///
/// # Arguments
///
/// * `commit_message` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message does not contain a JIRA issue key
/// * `None` - If the commit message contains a JIRA issue key
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint::JiraIssueKeyMissing;
///
/// // This should pass
/// let passing = CommitMessage::from("Subject\n\nBody\n\nJRA-123");
/// assert!(JiraIssueKeyMissing.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from("Subject\n\nBody");
/// assert!(JiraIssueKeyMissing.lint(&failing).is_some());
/// ```
///
/// # Errors
///
/// This function will never return an error, only an Option<Problem>
pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit_message, &JiraIssueKeyConfig)
}

fn lint_with_config(
    commit_message: &CommitMessage<'_>,
    _config: &JiraIssueKeyConfig,
) -> Option<Problem> {
    Some(commit_message)
        .filter(|commit| !has_jira_key(commit, &RE))
        .map(create_problem)
}

fn has_jira_key(commit_message: &CommitMessage<'_>, pattern: &Regex) -> bool {
    let comment_char = commit_message.get_comment_char();

    // Check if any non-comment line contains a JIRA key
    String::from(commit_message)
        .lines()
        .filter(|line| {
            !line
                .trim_start()
                .trim_start_matches(char::is_whitespace)
                .starts_with(comment_char.unwrap_or('#'))
        })
        .any(|line| pattern.is_match(line))
}

fn create_problem(commit_message: &CommitMessage) -> Problem {
    // Use ProblemBuilder instead of directly creating Problem
    ProblemBuilder::new(
        ERROR,
        HELP_MESSAGE,
        Code::JiraIssueKeyMissing,
        commit_message,
    )
    .with_label_at_last_line("No JIRA Issue Key")
    .with_url("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys")
    .build()
}

#[cfg(test)]
mod tests {
    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use mit_commit::CommitMessage;
    use quickcheck::TestResult;

    use super::*;

    #[test]
    fn test_jira_keys_in_comments_are_ignored() {
        test_has_missing_jira_issue_key(
            "An example commit\n\n# JRA-123 in comment",
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
                &"An example commit\n\n# JRA-123 in comment".into(),
                Some(vec![("No JIRA Issue Key".to_string(), 19, 20)]),
                Some("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys".parse().unwrap()),
            )).as_ref(),
        );
    }

    #[test]
    fn test_commit_with_jira_id_passes() {
        test_has_missing_jira_issue_key(
            "JRA-123 An example commit

This is an example commit
",
            None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

This is an JRA-123 example commit
",
            None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

JRA-123

This is an example commit
",
            None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

This is an example commit

JRA-123
",
            None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

This is an example commit

JR-123
",
            None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

This is an example commit

Relates-to: [JRA-123]
",
            None,
        );
        test_has_missing_jira_issue_key(
            "[JRA-123] An example commit

This is an example commit
",
            None,
        );
        test_has_missing_jira_issue_key(
            "An example commit

This is an [JRA-123] example commit
",
            None,
        );
    }

    #[test]
    fn test_commit_without_jira_id_fails() {
        let message_1 = "An example commit

This is an example commit
";
        test_has_missing_jira_issue_key(
            message_1,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
                &message_1.into(),
                Some(vec![("No JIRA Issue Key".to_string(), 19_usize, 25_usize)]),
                Some("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys".parse().unwrap()),
            )).as_ref(),
        );
        let message_2 = "An example commit

This is an example commit

A-123
";
        test_has_missing_jira_issue_key(
            message_2,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
                &message_2.into(),
                Some(vec![("No JIRA Issue Key".to_string(), 46_usize, 5_usize)]),
                Some("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys".parse().unwrap()),
            )).as_ref(),
        );
        let message_3 = "An example commit

This is an example commit

JRA-
";
        test_has_missing_jira_issue_key(
            message_3,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::JiraIssueKeyMissing,
                &message_3.into(),
                Some(vec![("No JIRA Issue Key".to_string(), 46_usize, 4_usize)]),
                Some("https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys".parse().unwrap()),
            )).as_ref(),
        );
    }

    #[test]
    fn test_error_report_formatting() {
        let message = "An example commit

This is an example commit
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "JiraIssueKeyMissing (https://support.atlassian.com/jira-software-cloud/docs/what-is-an-issue/#Workingwithissues-Projectkeys)

  x Your commit message is missing a JIRA Issue Key
   ,-[3:1]
 2 | 
 3 | This is an example commit
   : ^^^^^^^^^^^^|^^^^^^^^^^^^
   :             `-- No JIRA Issue Key
   `----
  help: It's important to add the issue key because it allows us to link code
        back to the motivations for doing it, and in some cases provide an
        audit trail for compliance purposes.
        
        You can fix this by adding a key like `JRA-123` to the commit message
" .to_string();
        assert_eq!(
            actual, expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    fn fmt_report(diag: &Report) -> String {
        let mut out = String::new();
        GraphicalReportHandler::new_themed(GraphicalTheme::none())
            .with_width(80)
            .with_links(false)
            .render_report(&mut out, diag.as_ref())
            .unwrap();
        out
    }

    fn test_has_missing_jira_issue_key(message: &str, expected: Option<&Problem>) {
        let actual = lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[derive(Debug, Clone)]
    struct CommitWithoutJira(String);

    impl quickcheck::Arbitrary for CommitWithoutJira {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            // Generate a commit message guaranteed to lack JIRA issue keys
            let subject = String::arbitrary(g)
                .replace(|c: char| c.is_ascii_uppercase() || c == '-', "")
                .replace("JRA", "")
                .replace("PROJ", "");

            let mut body = String::new();
            for _ in 0..=(usize::arbitrary(g) % 5) {
                let word = String::arbitrary(g)
                    .replace(|c: char| c.is_ascii_uppercase() || c.is_ascii_digit(), "")
                    .replace('-', "");
                body.push_str(&word);
                body.push(' ');
            }

            Self(format!("{subject}\n\n{body}"))
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_without_jira_id_fail(commit: CommitWithoutJira) -> TestResult {
        let message = CommitMessage::from(commit.0);
        let result = lint(&message);
        TestResult::from_bool(result.is_some())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_with_jira_id_pass(
        before: Option<String>,
        characters: String,
        numbers: usize,
        after: Option<String>,
    ) -> TestResult {
        if characters.chars().count() < 2
            || characters
                .chars()
                .any(|x| !x.is_ascii_alphabetic() || !x.is_uppercase())
        {
            return TestResult::discard();
        }

        let message = CommitMessage::from(format!(
            "{}{}-{}{}\n# comment",
            before.map(|x| format!("{x} ")).unwrap_or_default(),
            characters,
            numbers,
            after.map(|x| format!(" {x} ")).unwrap_or_default(),
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }
}
