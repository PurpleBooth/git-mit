use mit_commit::CommitMessage;

use crate::model::{Code, Problem, ProblemBuilder};

/// Canonical lint ID
pub const CONFIG: &str = "subject-line-not-capitalized";
/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "The subject line is a title, and as such should be \
                            capitalised.\n\nYou can fix this by capitalising the first character \
                            in the subject";
/// Description of the problem
pub const ERROR: &str = "Your commit message is missing a capital letter";

fn has_problem(commit_message: &CommitMessage<'_>) -> bool {
    commit_message
        .get_subject()
        .chars()
        .find(|x| !x.is_whitespace())
        .is_some_and(char::is_lowercase)
}

pub struct SubjectNotCapitalizedConfig;
impl Default for SubjectNotCapitalizedConfig {
    fn default() -> Self {
        Self
    }
}

pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit_message, &SubjectNotCapitalizedConfig)
}

fn lint_with_config(
    commit_message: &CommitMessage<'_>,
    _config: &SubjectNotCapitalizedConfig,
) -> Option<Problem> {
    Some(commit_message)
        .filter(|commit| has_problem(commit))
        .map(create_problem)
}

fn create_problem(commit_message: &CommitMessage) -> Problem {
    let position = commit_message
        .get_subject()
        .chars()
        .filter(|x| x.is_whitespace())
        .count()
        .saturating_sub(2);

    ProblemBuilder::new(
        ERROR,
        HELP_MESSAGE,
        Code::SubjectNotCapitalized,
        commit_message,
    )
    .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines")
    .with_label("Not capitalised", position, 1)
    .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::option::Option::None;

    use crate::{Code, Problem};
    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use mit_commit::CommitMessage;
    use quickcheck::TestResult;

    #[test]
    fn test_capitalized_subject_passes() {
        run_test("Subject Line", None);
    }

    #[test]
    fn test_lowercase_subject_fails() {
        run_test(
            "subject line
",
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectNotCapitalized,
                &"subject line
"
                    .into(),
                Some(vec![("Not capitalised".to_string(), 0_usize, 1_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
            )).as_ref(),
        );
    }

    #[test]
    fn test_leading_space_with_lowercase_subject_fails() {
        run_test(
            "  subject line",
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectNotCapitalized,
                &CommitMessage::from("  subject line"),
                Some(vec![("Not capitalised".to_string(), 1_usize, 1_usize)]),
                Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".to_string()),
            )).as_ref(),
        );
    }

    #[test]
    fn test_numeric_subject_passes() {
        run_test(
            "1234567
", None,
        );
    }

    #[test]
    fn test_unicode_titlecase_character() {
        // "ǅ" (U+01C5 LATIN CAPITAL LETTER D WITH SMALL LETTER Z WITH CARON) is a
        // titlecase character (General Category Lt): both is_lowercase() and
        // is_uppercase() return false for it, so the lint must not flag it.
        run_test("ǅ", None);
    }

    #[test]
    fn test_error_formatting_matches_expected_output() {
        let message = "  an example commit\n\nexample";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));

        // Instead of comparing the exact strings, we'll normalize the whitespace
        // by removing all whitespace and then comparing
        let normalize_whitespace =
            |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

        let actual_normalized = normalize_whitespace(&actual);
        let expected_normalized = normalize_whitespace("SubjectNotCapitalized (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit message is missing a capital letter
   ,-[1:3]
 1 |   an example commit
   :   |
   :   `-- Not capitalised
 2 | 
   `----
  help: The subject line is a title, and as such should be capitalised.

        You can fix this by capitalising the first character in the subject
");

        assert_eq!(
            actual_normalized, expected_normalized,
            "Message {message:?} should have returned a string equivalent to the expected output after normalizing whitespace"
        );
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_lowercase_first_character_always_fails(commit_message_body: String) -> TestResult {
        match commit_message_body
            .chars()
            .take_while(|x| *x != '\n')
            .find(|x| !x.is_whitespace())
        {
            None => return TestResult::discard(),
            Some(char) => {
                // The lint flags a commit only when the first non-whitespace
                // character is lowercase, so discard anything that isn't. This
                // includes titlecase characters like "ǅ" (U+01C5, General
                // Category Lt) for which both is_lowercase() and is_uppercase()
                // return false — the lint won't flag them.
                if !char.is_lowercase() {
                    return TestResult::discard();
                }
            }
        }

        let message = CommitMessage::from(format!("{commit_message_body}\n# commit"));
        let result = lint(&message);
        let b = result.is_some();
        TestResult::from_bool(b)
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_uppercase_first_character_always_passes(commit_message_body: String) -> TestResult {
        if commit_message_body.starts_with('#') {
            return TestResult::discard();
        }

        match commit_message_body
            .chars()
            .take_while(|x| *x != '\n')
            .find(|x| !x.is_whitespace())
        {
            None => return TestResult::discard(),
            Some(char) => {
                if char.is_lowercase() {
                    return TestResult::discard();
                }
            }
        }

        let message = CommitMessage::from(format!("{commit_message_body}\n# commit"));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
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
        let actual = lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }
}
