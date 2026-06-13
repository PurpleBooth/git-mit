use mit_commit::CommitMessage;
use quickcheck::{Arbitrary, Gen};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::model::{Code, Problem, ProblemBuilder};

/// Canonical lint ID
pub const CONFIG: &str = "not-emoji-log";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to follow the emoji log style when creating your commit message. By using this \
style we can automatically generate changelogs.

You can fix it using one of the prefixes:


\u{1f4e6} NEW:
\u{1f44c} IMPROVE:
\u{1f41b} FIX:
\u{1f4d6} DOC:
\u{1f680} RELEASE:
\u{1f916} TEST:
\u{203c}\u{fe0f} BREAKING:";
/// Description of the problem
pub const ERROR: &str = "Your commit message isn't in emoji log style";

/// Configuration for emoji log linting
pub struct EmojiLogConfig;

impl Default for EmojiLogConfig {
    fn default() -> Self {
        Self
    }
}

/// Checks if the commit message follows the emoji log style
///
/// # Arguments
///
/// * `commit_message` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message does not follow the emoji log style
/// * `None` - If the commit message follows the emoji log style
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint::NotEmojiLog;
///
/// // This should pass
/// let passing = CommitMessage::from("📦 NEW: Add new feature");
/// assert!(NotEmojiLog.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from("Add new feature");
/// assert!(NotEmojiLog.lint(&failing).is_some());
/// ```
///
/// # Errors
///
/// This function will never return an error, only an Option<Problem>
pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit_message, &EmojiLogConfig)
}

fn lint_with_config(
    commit_message: &CommitMessage<'_>,
    _config: &EmojiLogConfig,
) -> Option<Problem> {
    Some(commit_message)
        .filter(|commit| has_problem(commit))
        .map(create_problem)
}

fn has_problem(commit_message: &CommitMessage<'_>) -> bool {
    // Check if the commit message starts with any of the valid emoji log prefixes
    !Prefix::iter().any(|prefix| {
        commit_message
            .get_subject()
            .to_string()
            .starts_with(&String::from(prefix))
    })
}

fn create_problem(commit_message: &CommitMessage) -> Problem {
    // Create a problem with appropriate labels
    let commit_text = String::from(commit_message.clone());
    let first_line_length = commit_text.lines().next().map(str::len).unwrap_or_default();

    ProblemBuilder::new(ERROR, HELP_MESSAGE, Code::NotEmojiLog, commit_message)
        .with_label("Not emoji log", 0, first_line_length)
        .with_url("https://github.com/ahmadawais/Emoji-Log")
        .build()
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, EnumIter)]
pub enum Prefix {
    Fix,
    New,
    Improve,
    Doc,
    Release,
    Test,
    Breaking,
}

impl From<Prefix> for String {
    fn from(input: Prefix) -> Self {
        match input {
            Prefix::Fix => Self::from("\u{1f41b} FIX: "),
            Prefix::New => Self::from("\u{1f4e6} NEW: "),
            Prefix::Improve => Self::from("\u{1f44c} IMPROVE: "),
            Prefix::Doc => Self::from("\u{1f4d6} DOC: "),
            Prefix::Release => Self::from("\u{1f680} RELEASE: "),
            Prefix::Test => Self::from("\u{1f916} TEST: "),
            Prefix::Breaking => Self::from("\u{203c}\u{fe0f} BREAKING: "),
        }
    }
}

impl Arbitrary for Prefix {
    fn arbitrary(g: &mut Gen) -> Self {
        *g.choose(&Self::iter().collect::<Vec<_>>()).unwrap()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        quickcheck::empty_shrinker()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Code, Problem};
    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use mit_commit::CommitMessage;
    use quickcheck::TestResult;
    use std::option::Option::None;
    use strum::IntoEnumIterator;

    #[test]
    fn new() {
        run_lint(
            "\u{1f4e6} NEW: An example commit

This is an example commit
",
            None,
        );
    }

    #[test]
    fn improve() {
        run_lint(
            "\u{1f44c} IMPROVE: An example commit

This is an example commit
",
            None,
        );
    }

    #[test]
    fn fix() {
        run_lint(
            "\u{1f41b} FIX: An example commit

This is an example commit
",
            None,
        );
    }

    #[test]
    fn docs() {
        run_lint(
            "\u{1f4d6} DOC: An example commit

This is an example commit
",
            None,
        );
    }

    #[test]
    fn release() {
        run_lint(
            "\u{1f680} RELEASE: An example commit

This is an example commit
",
            None,
        );
    }

    #[test]
    fn test() {
        run_lint(
            "\u{1f916} TEST: An example commit

This is an example commit
",
            None,
        );
    }

    #[test]
    fn bc() {
        run_lint(
            "\u{203c}\u{fe0f} BREAKING: An example commit

This is an example commit
",
            None,
        );
    }

    #[test]
    fn no_gap() {
        let message = "\u{203c}\u{fe0f} BREAKING:An example commit

This is an example commit
";
        run_lint(
            message,
            Some(&Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotEmojiLog,
                &message.into(),
                Some(vec![("Not emoji log".to_string(), 0_usize, 33_usize)]),
                Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
            )),
        );
    }

    #[test]
    fn unknown_emoji() {
        let message = "\u{1f408} UNKNOWN: An example commit

This is an example commit
";
        run_lint(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotEmojiLog,
                &message.into(),
                Some(vec![("Not emoji log".to_string(), 0_usize, 31_usize)]),
                Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn not_emoji() {
        let message = "An example commit

This is an example commit
";
        run_lint(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotEmojiLog,
                &message.into(),
                Some(vec![("Not emoji log".to_string(), 0_usize, 17_usize)]),
                Some("https://github.com/ahmadawais/Emoji-Log".to_string()),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn formatting() {
        let message = "An example commit

This is an example commit
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "NotEmojiLog (https://github.com/ahmadawais/Emoji-Log)

  x Your commit message isn't in emoji log style
   ,-[1:1]
 1 | An example commit
   : ^^^^^^^^|^^^^^^^^
   :         `-- Not emoji log
 2 | 
   `----
  help: It's important to follow the emoji log style when creating your commit
        message. By using this style we can automatically generate changelogs.
        
        You can fix it using one of the prefixes:
        
        
        \u{1f4e6} NEW:
        \u{1f44c} IMPROVE:
        \u{1f41b} FIX:
        \u{1f4d6} DOC:
        \u{1f680} RELEASE:
        \u{1f916} TEST:
        \u{203c}\u{fe0f} BREAKING:
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

    fn run_lint(message: &str, expected: Option<&Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[test]
    fn emoji_log_prefixes_new() {
        let input: Prefix = Prefix::New;

        assert_eq!(String::from(input), "\u{1f4e6} NEW: ".to_string());
    }

    #[test]
    fn emoji_log_prefixes_improve() {
        let input: Prefix = Prefix::Improve;

        assert_eq!(String::from(input), "\u{1f44c} IMPROVE: ".to_string());
    }

    #[test]
    fn emoji_log_prefixes_fix() {
        let input: Prefix = Prefix::Fix;

        assert_eq!(String::from(input), "\u{1f41b} FIX: ".to_string());
    }

    #[test]
    fn emoji_log_prefixes_docs() {
        let input: Prefix = Prefix::Doc;

        assert_eq!(String::from(input), "\u{1f4d6} DOC: ".to_string());
    }

    #[test]
    fn emoji_log_prefixes_release() {
        let input: Prefix = Prefix::Release;

        assert_eq!(String::from(input), "\u{1f680} RELEASE: ".to_string());
    }

    #[test]
    fn emoji_log_prefixes_test() {
        let input: Prefix = Prefix::Test;

        assert_eq!(String::from(input), "\u{1f916} TEST: ".to_string());
    }

    #[test]
    fn emoji_log_prefixes_breaking() {
        let input: Prefix = Prefix::Breaking;

        assert_eq!(
            String::from(input),
            "\u{203c}\u{fe0f} BREAKING: ".to_string()
        );
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn get_prefixes(prefix: Prefix) -> bool {
        Prefix::iter().any(|x| x == prefix)
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn success_check(prefix: Prefix, subject: String, body: Option<String>) -> TestResult {
        if subject.contains('\n') {
            return TestResult::discard();
        }

        let message = CommitMessage::from(format!(
            "{}{}{}\n# Comment",
            String::from(prefix),
            subject,
            body.map(|x| format!("\n\n{x}")).unwrap_or_default()
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn fail_check(commit: String) -> TestResult {
        if Prefix::iter()
            .map(String::from)
            .any(|x| commit.starts_with(&x))
        {
            return TestResult::discard();
        }

        let message = CommitMessage::from(commit);
        let result = lint(&message);
        TestResult::from_bool(result.is_some())
    }
}
