use mit_commit::CommitMessage;

use crate::model::{Code, Problem, ProblemBuilder};

/// Canonical lint ID
pub const CONFIG: &str = "body-wider-than-72-characters";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to keep the body of the commit narrower than 72 \
                            characters because when you look at the git log, that's where it \
                            truncates the message. This means that people won't get the entirety \
                            of the information in your commit.\n\nYou can fix this by making the \
                            lines in your body no more than 72 characters";
/// Description of the problem
pub const ERROR: &str = "Your commit has a body wider than 72 characters";

/// Character limit for body width
pub const CHARACTER_LIMIT: usize = 72;

pub struct BodyWidthConfig {
    /// Maximum allowed width for body lines
    pub character_limit: usize,
}

impl Default for BodyWidthConfig {
    fn default() -> Self {
        Self {
            character_limit: CHARACTER_LIMIT,
        }
    }
}

/// Checks if the commit message body has lines wider than 72 characters
///
/// # Arguments
///
/// * `commit` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message body has lines wider than 72 characters
/// * `None` - If the commit message body has no lines wider than 72 characters
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint;
///
/// // This should pass
/// let passing = CommitMessage::from("Subject\n\nBody that is less than 72 chars wide");
/// assert!(Lint::BodyWiderThan72Characters.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from(
///     "Subject\n\nThis line is way too long and exceeds the 72 character limit by quite a bit actually"
/// );
/// assert!(Lint::BodyWiderThan72Characters.lint(&failing).is_some());
/// ```
/// Configuration for body width linting
pub fn lint(commit: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit, &BodyWidthConfig::default())
}

fn lint_with_config(commit: &CommitMessage, config: &BodyWidthConfig) -> Option<Problem> {
    Some(commit)
        .filter(|commit| has_problem(commit, config.character_limit))
        .map(|commit| create_problem(commit, config.character_limit))
}

fn has_problem(commit: &CommitMessage<'_>, limit: usize) -> bool {
    commit
        .get_body()
        .to_string()
        .lines()
        .any(|line| line.chars().count() > limit)
}

fn create_problem(commit: &CommitMessage, limit: usize) -> Problem {
    let commit_text: String = commit.into();
    let scissors_start_line = calculate_scissors_start_line(commit, &commit_text);
    let comment_char = commit.get_comment_char().map(|x| format!("{x} "));

    let mut builder = ProblemBuilder::new(
        ERROR,
        HELP_MESSAGE,
        Code::BodyWiderThan72Characters,
        commit
    )
    .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines");

    // Add labels for all lines that exceed the limit
    for (line_index, line) in commit_text.lines().enumerate() {
        // Skip lines after scissors line or comment lines
        if line_index > scissors_start_line
            || comment_char.as_ref().is_some_and(|cc| line.starts_with(cc))
        {
            continue;
        }

        // Add label if line exceeds limit
        builder = builder.with_label_for_line(&commit_text, line_index, line, limit, "Too long");
    }

    builder.build()
}

fn calculate_scissors_start_line(commit: &CommitMessage, commit_text: &str) -> usize {
    commit_text.lines().count()
        - commit
            .get_scissors()
            .map_or(0, |s| String::from(s).lines().count())
}

#[cfg(test)]
mod tests {
    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use quickcheck::TestResult;

    use super::*;

    #[test]
    fn test_body_with_width_equal_to_limit_passes() {
        test_body_wider_than_72_characters(&format!("Subject\n\n{}", "x".repeat(72)), None);
    }

    #[test]
    fn test_commit_with_no_body_passes() {
        test_body_wider_than_72_characters("Subject", None);
    }

    #[test]
    fn test_body_within_limit_with_long_comments_passes() {
        let message = "Remove duplicated function

The function got skipped in thee previous round of refactoring
# Short (50 chars or less) summary of changes
#
# More detailed explanatory text, if necessary.  Wrap it to
# about 72 characters or so.  In some contexts, the first
# line is treated as the subject of an email and the rest of
# the text as the body.  The blank line separating the
# summary from the body is critical (unless you omit the body
# entirely); tools like rebase can get confused if you run
# the two together.
#
# Further paragraphs come after blank lines.
#
#   - Bullet points are okay, too
#
#   - Typically a hyphen or asterisk is used for the bullet,
#     preceded by a single space, with blank lines in
#     between, but conventions vary here

# Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00C4}nderungen ein. \
 Zeilen,
# die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
# bricht den Commit ab.
#
# Auf Branch character-limit
# Zum Commit vorgemerkte \u{00C4}nderungen:
#       ge\u{00E4}ndert:       \
 mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs
#
# ------------------------ >8 ------------------------
# \u{00C4}ndern oder entfernen Sie nicht die obige Zeile.
# Alles unterhalb von ihr wird ignoriert.
diff --git a/mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs \
 b/mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs
index 5a83784..ebaee48 100644
--- a/mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs
+++ b/mit-commit-message-lints/src/lints/model/missing_pivotal_tracker_id.rs
-fn has_missing_pivotal_tracker_id(commit_message: &CommitMessage) -> bool {
-    has_no_pivotal_tracker_id(commit_message)
-}
-
 fn has_no_pivotal_tracker_id(text: &CommitMessage) -> bool {
     let re = Regex::new(REGEX_PIVOTAL_TRACKER_ID).unwrap();
     !text.matches_pattern(&re)
 }

 pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
-    if has_missing_pivotal_tracker_id(commit_message) {
+    if has_no_pivotal_tracker_id(commit_message) {
         Some(Problem::new(
             PIVOTAL_TRACKER_HELP.into(),
             Code::PivotalTrackerIdMissing,


";
        test_body_wider_than_72_characters(&format!("{}\n\n{message}", "x".repeat(72)), None);
    }

    #[test]
    fn test_body_exceeding_width_limit_fails() {
        let message = format!("Subject\n\n{}", "x".repeat(73));
        let commit = CommitMessage::from(message.clone());

        let expected_problem = ProblemBuilder::new(
            ERROR,
            HELP_MESSAGE,
            Code::BodyWiderThan72Characters,
            &commit
        )
        .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines")
        .with_label_for_line(&message, 2, &"x".repeat(73), 72, "Too long")
        .build();

        test_body_wider_than_72_characters(&message, Some(&expected_problem));
    }

    #[test]
    fn test_body_exceeding_width_limit_by_multiple_chars_fails() {
        let message = format!("Subject\n\n{}", "x".repeat(75));
        let commit = CommitMessage::from(message.clone());

        let expected_problem = ProblemBuilder::new(
            ERROR,
            HELP_MESSAGE,
            Code::BodyWiderThan72Characters,
            &commit
        )
        .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines")
        .with_label_for_line(&message, 2, &"x".repeat(75), 72, "Too long")
        .build();

        test_body_wider_than_72_characters(&message, Some(&expected_problem));
    }

    #[test]
    fn test_body_with_multiple_long_lines_fails() {
        let message = format!("Subject\n\n{}\n{}", "x".repeat(73), "y".repeat(73));
        let commit = CommitMessage::from(message.clone());

        let expected_problem = ProblemBuilder::new(
            ERROR,
            HELP_MESSAGE,
            Code::BodyWiderThan72Characters,
            &commit
        )
        .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines")
        .with_label_for_line(&message, 2, &"x".repeat(73), 72, "Too long")
        .with_label_for_line(&message, 3, &"y".repeat(73), 72, "Too long")
        .build();

        test_body_wider_than_72_characters(&message, Some(&expected_problem));
    }

    #[test]
    fn test_body_with_some_lines_exceeding_limit_fails() {
        let message = format!("Subject\n\nx\n{}\nx\n", "x".repeat(73));
        let commit = CommitMessage::from(message.clone());

        let expected_problem = ProblemBuilder::new(
            ERROR,
            HELP_MESSAGE,
            Code::BodyWiderThan72Characters,
            &commit
        )
        .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines")
        .with_label_for_line(&message, 3, &"x".repeat(73), 72, "Too long")
        .build();

        test_body_wider_than_72_characters(&message, Some(&expected_problem));
    }

    #[test]
    fn test_body_with_last_line_exceeding_limit_fails() {
        let message = format!("Subject\n\n{}", "x".repeat(73));
        let commit = CommitMessage::from(message.clone());

        let expected_problem = ProblemBuilder::new(
            ERROR,
            HELP_MESSAGE,
            Code::BodyWiderThan72Characters,
            &commit
        )
        .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines")
        .with_label_for_line(&message, 2, &"x".repeat(73), 72, "Too long")
        .build();

        test_body_wider_than_72_characters(&message, Some(&expected_problem));
    }

    #[test]
    fn test_lines_after_scissors_are_ignored() {
        let message = [
            "Subject",
            "",
            "x",
            &"x".repeat(72),
            "# ------------------------ >8 ------------------------",
            &"x".repeat(73),
        ]
        .join("\n");
        test_body_wider_than_72_characters(&message, None);
    }

    fn test_body_wider_than_72_characters(message: &str, expected: Option<&Problem>) {
        let actual = lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[test]
    fn test_error_report_formatting() {
        let message = format!(
            "Subject\n\nx\n{}\nx\n{}\nx\n",
            "x".repeat(73),
            "x".repeat(80)
        );
        let problem = lint(&CommitMessage::from(message.clone()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "BodyWiderThan72Characters (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit has a body wider than 72 characters
   ,-[4:73]
 3 | x
 4 | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   :                                                                         |
   :                                                                         `-- Too long
 5 | x
 6 | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   :                                                                         ^^^^|^^^
   :                                                                             `-- Too long
 7 | x
   `----
  help: It's important to keep the body of the commit narrower than 72
        characters because when you look at the git log, that's where it
        truncates the message. This means that people won't get the entirety
        of the information in your commit.
        
        You can fix this by making the lines in your body no more than 72
        characters
".to_string();
        assert_eq!(
            actual, expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[test]
    fn test_error_highlights_exclude_scissors_section() {
        let message = [
            "Subject",
            "",
            "x",
            &"x".repeat(73),
            "# ------------------------ >8 ------------------------",
            &"x".repeat(73),
        ]
        .join("\n");

        let problem = lint(&CommitMessage::from(message.clone()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "BodyWiderThan72Characters (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit has a body wider than 72 characters
   ,-[4:73]
 3 | x
 4 | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   :                                                                         |
   :                                                                         `-- Too long
 5 | # ------------------------ >8 ------------------------
   `----
  help: It's important to keep the body of the commit narrower than 72
        characters because when you look at the git log, that's where it
        truncates the message. This means that people won't get the entirety
        of the information in your commit.
        
        You can fix this by making the lines in your body no more than 72
        characters
".to_string();
        assert_eq!(
            actual, expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[test]
    fn test_error_highlights_exclude_comment_lines() {
        let message = [
            "Subject",
            "",
            "x",
            &"x".repeat(73),
            &format!("# {}", "x".repeat(71)),
            "# ------------------------ >8 ------------------------",
            &"x".repeat(73),
        ]
        .join("\n");

        let problem = lint(&CommitMessage::from(message.clone()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "BodyWiderThan72Characters (https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines)

  x Your commit has a body wider than 72 characters
   ,-[4:73]
 3 | x
 4 | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   :                                                                         |
   :                                                                         `-- Too long
 5 | # xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
   `----
  help: It's important to keep the body of the commit narrower than 72
        characters because when you look at the git log, that's where it
        truncates the message. This means that people won't get the entirety
        of the information in your commit.
        
        You can fix this by making the lines in your body no more than 72
        characters
".to_string();
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

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn success_check(input: Vec<u8>) -> TestResult {
        // Clean and normalize test input through several transformations:
        // 1. Convert raw bytes to valid UTF-8, replacing invalid sequences
        let utf8_cleaned = String::from_utf8_lossy(&input).into_owned();

        // Ensure we have a valid commit structure with non-empty subject and body separator
        if utf8_cleaned.is_empty()
            || utf8_cleaned.starts_with('\n')
            || !utf8_cleaned.contains("\n\n")
        {
            return TestResult::discard();
        }

        // Split into subject and body parts
        let parts: Vec<&str> = utf8_cleaned.split("\n\n").collect();
        if parts.len() != 2 || parts[0].trim().is_empty() {
            return TestResult::discard();
        }

        // Check body lines (excluding comments) for length
        let body = parts[1];
        let mut lines_valid = true;

        for line in body.split('\n') {
            // Skip comment lines
            if line.starts_with('#') {
                continue;
            }

            // Check actual byte length like the linter does
            if line.len() > 72 {
                lines_valid = false;
                break;
            }
        }

        if !lines_valid {
            return TestResult::discard();
        }

        let message = CommitMessage::from(utf8_cleaned);
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }

    #[test]
    fn test_unicode_character_handling() {
        // This string has 73 Unicode characters in a single line (146 bytes)
        let message = format!("Subject\n\n{}", "\u{1f600}".repeat(73));
        let commit = CommitMessage::from(message.clone());

        let expected_problem = ProblemBuilder::new(
            ERROR,
            HELP_MESSAGE,
            Code::BodyWiderThan72Characters,
            &commit
        )
        .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines")
        .with_label_for_line(&message, 2, &"\u{1f600}".repeat(73), 72, "Too long")
        .build();

        test_body_wider_than_72_characters(&message, Some(&expected_problem));
    }

    #[test]
    fn test_null_byte_handling() {
        let message = "\0\n\n\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
        let commit = CommitMessage::from(message);

        let expected_problem = ProblemBuilder::new(
            ERROR,
            HELP_MESSAGE,
            Code::BodyWiderThan72Characters,
            &commit
        )
        .with_url("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines")
        .with_label_for_line(message, 2, &"\0".repeat(73), 72, "Too long")
        .build();

        test_body_wider_than_72_characters(message, Some(&expected_problem));
    }

    #[test]
    fn test_custom_character_limit() {
        // Create a custom config with a different character limit
        let config = BodyWidthConfig {
            character_limit: 50,
        };

        // Test with a line exactly at the custom limit
        let message = format!("Subject\n\n{}", "x".repeat(50));
        let commit = CommitMessage::from(message);
        let result = lint_with_config(&commit, &config);
        assert!(result.is_none(), "Line at custom limit should pass");

        // Test with a line exceeding the custom limit
        let message = format!("Subject\n\n{}", "x".repeat(51));
        let commit = CommitMessage::from(message);
        let result = lint_with_config(&commit, &config);
        assert!(result.is_some(), "Line exceeding custom limit should fail");

        // Verify the error message references the custom limit
        if let Some(problem) = result {
            let report = Report::new(problem);
            let formatted = fmt_report(&report);
            assert!(
                formatted.contains("Too long"),
                "Error should indicate the line is too long"
            );
        }
    }

    #[derive(Debug, Clone)]
    struct CommitBody(String);

    impl quickcheck::Arbitrary for CommitBody {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            // Generate body lines with some lines over 72 chars but no invalid characters
            let line_count = usize::arbitrary(g) % 20 + 1;
            let mut body = String::new();

            for _ in 0..line_count {
                // Create guaranteed overlong line for testing failure cases
                let padding = 0; // Don't pad since it might make line valid again
                let overlong = "x".repeat(73);

                // Replace entire line with guaranteed overlong content
                let line = format!("{}{}", overlong, " ".repeat(padding));

                body.push_str(&line);
                body.push('\n');
            }

            // Build full commit message with valid structure
            Self(format!("Valid subject\n\n{}", body.trim_end()))
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn fail_check(commit: CommitBody) -> TestResult {
        let commit = commit.0;

        // Split into subject and body parts
        let parts: Vec<&str> = commit.split("\n\n").collect();
        if parts.len() != 2 || parts[0].trim().is_empty() {
            return TestResult::discard();
        }

        // Check body lines (excluding comments) for at least one line over CHARACTER limit
        let body = parts[1];
        if body
            .lines()
            .filter(|line| !line.starts_with('#'))
            .all(|line| line.chars().count() <= 72)
        {
            return TestResult::discard();
        }

        let message = CommitMessage::from(commit);
        let result = lint(&message);
        TestResult::from_bool(result.is_some())
    }
}
