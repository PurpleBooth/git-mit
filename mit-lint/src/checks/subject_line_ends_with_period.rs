use mit_commit::CommitMessage;

use crate::model::{Code, Problem, ProblemBuilder};

/// Canonical lint ID
pub const CONFIG: &str = "subject-line-ends-with-period";
/// Description of the problem
pub const ERROR: &str = "Your commit message ends with a period";
/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to keep your commits short, because we only have a \
                            limited number of characters to use (72) before the subject line is \
                            truncated. Full stops aren't normally in subject lines, and take up \
                            an extra character, so we shouldn't use them in commit message \
                            subjects.\n\nYou can fix this by removing the period";

/// Configuration for subject line ending with period linting
pub struct SubjectLineEndsWithPeriodConfig;

impl Default for SubjectLineEndsWithPeriodConfig {
    fn default() -> Self {
        Self
    }
}

/// Checks if the commit message subject line ends with a period
///
/// # Arguments
///
/// * `commit_message` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message subject line ends with a period
/// * `None` - If the commit message subject line does not end with a period
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint::SubjectEndsWithPeriod;
///
/// // This should pass
/// let passing = CommitMessage::from("Subject Line");
/// assert!(SubjectEndsWithPeriod.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from("Subject Line.");
/// assert!(SubjectEndsWithPeriod.lint(&failing).is_some());
/// ```
///
/// # Errors
///
/// This function will never return an error, only an Option<Problem>
pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit_message, &SubjectLineEndsWithPeriodConfig)
}

fn lint_with_config(
    commit_message: &CommitMessage<'_>,
    _config: &SubjectLineEndsWithPeriodConfig,
) -> Option<Problem> {
    Some(commit_message)
        .filter(|commit| has_problem(commit))
        .map(create_problem)
}

fn has_problem(commit_message: &CommitMessage<'_>) -> bool {
    commit_message
        .get_subject()
        .to_string()
        .trim_end()
        .chars()
        .next_back()
        .is_some_and(|x| x == '.')
}

fn create_problem(commit_message: &CommitMessage) -> Problem {
    // Create a problem with appropriate labels
    let subject = commit_message.get_subject().to_string();

    // Calculate the position of the period(s)
    let period_position = subject.len()
        - subject
            .chars()
            .rev()
            .filter(|ch| ch == &'.' || ch.is_whitespace())
            .count()
            .saturating_sub(2);

    // Count how many periods there are
    let period_count = subject
        .chars()
        .rev()
        .filter(|ch| !ch.is_whitespace())
        .take_while(|ch| ch == &'.')
        .count();

    ProblemBuilder::new(
        ERROR,
        HELP_MESSAGE,
        Code::SubjectEndsWithPeriod,
        commit_message,
    )
    .with_label("Unneeded period", period_position, period_count)
    .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines")
    .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Code, Problem};
    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use mit_commit::CommitMessage;
    use quickcheck::TestResult;
    use std::option::Option::None;

    #[test]
    fn subject_does_not_end_with_period() {
        run_test(
            "Subject Line
",
            None,
        );
    }

    #[test]
    fn subject_ends_with_period() {
        let message = "Subject Line.
";
        run_test(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
                &message.into(),
                Some(vec![("Unneeded period".to_string(), 13_usize, 1_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
            )).as_ref(),
            );
    }

    #[test]
    fn subject_has_period_then_whitespace() {
        let message = "Subject Line. ";
        run_test(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
                &message.into(),
                Some(vec![("Unneeded period".to_string(), 13_usize, 1_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
            )).as_ref(),
            );
    }

    #[test]
    fn subject_has_multiple_periods() {
        let message = "Subject Line... ";
        run_test(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
                &message.into(),
                Some(vec![("Unneeded period".to_string(), 13_usize, 3_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
            )).as_ref(),
            );
    }

    #[test]
    fn subject_is_just_a_period() {
        let message = ".\n# bla";
        run_test(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
                &message.into(),
                Some(vec![("Unneeded period".to_string(), 1_usize, 1_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
            )).as_ref(),
        );
    }

    #[test]
    fn formatting() {
        let message = "An example commit... 

This is an example commit
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "SubjectEndsWithPeriod (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit message ends with a period
   ,-[1:18]
 1 | An example commit... 
   :                  ^|^
   :                   `-- Unneeded period
 2 | 
   `----
  help: It's important to keep your commits short, because we only have a
        limited number of characters to use (72) before the subject line is
        truncated. Full stops aren't normally in subject lines, and take up an
        extra character, so we shouldn't use them in commit message subjects.
        
        You can fix this by removing the period
"
            .to_string();
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

    fn run_test(message: &str, expected: Option<&Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn success_check(subject: String, body: Option<String>) -> TestResult {
        if subject.trim_end().ends_with('.') {
            return TestResult::discard();
        }
        if subject.contains('\n') {
            return TestResult::discard();
        }
        let message = CommitMessage::from(format!(
            "{}{}",
            subject,
            body.map(|x| format!("\n\n{x}")).unwrap_or_default()
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn failure_check(subject: String, body: Option<String>) -> TestResult {
        if subject.contains('\n') {
            return TestResult::discard();
        }
        if subject.starts_with('#') {
            return TestResult::discard();
        }
        if !subject.trim_end().ends_with('.') {
            return TestResult::discard();
        }
        let message = CommitMessage::from(format!(
            "{}{}\n# bla",
            subject,
            body.map(|x| format!("\n\n{x}")).unwrap_or_default()
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_some())
    }
}
