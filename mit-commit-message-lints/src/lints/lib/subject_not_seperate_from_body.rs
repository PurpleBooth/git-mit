use indoc::indoc;

use crate::lints::lib::problem::Code;
use crate::lints::lib::{CommitMessage, Problem};

pub(crate) const CONFIG: &str = "subject-not-separated-from-body";

const HELP_MESSAGE: &str = indoc!(
    "
    Your commit message is missing a blank line between the subject and the body

    To fix this separate subject from body with a blank line
    "
);

const SIZE_OF_SUBJECT: usize = 1;

fn has_problem(commit_message: &CommitMessage) -> bool {
    let line_count = commit_message.content_line_count();

    match line_count {
        1 => false,
        2 => true,
        _ => commit_message.get_body().lines().count() + SIZE_OF_SUBJECT == line_count,
    }
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(commit_message) {
        Some(Problem::new(
            HELP_MESSAGE.into(),
            Code::SubjectNotSeparateFromBody,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn with_gutter() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                An example commit

                This is an example commit
                "
            ),
            &None,
        );
        test_subject_not_seperate_from_body(
            indoc!(
                "
                Another example

                Disabling this specific lint - Co-authored

                Co-authored-by: Someone Else <someone@example.com>
                Co-authored-by: Someone Else <someone@example.com>
                "
            ),
            &None,
        );
    }

    #[test]
    fn single_line_with_trailing_newline() {
        test_subject_not_seperate_from_body("An example commit\n", &None);
    }

    #[test]
    fn single_line_with_long_comments() {
        test_subject_not_seperate_from_body(
            indoc!(
            "
            Remove duplicated function
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

            # Bitte geben Sie eine Commit-Beschreibung f\u{00FC}r Ihre \u{00C4}nderungen ein. Zeilen,
            # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung
            # bricht den Commit ab.
            #
            # Auf Branch character-limit
            # Zum Commit vorgemerkte \u{00C4}nderungen:
            #	ge\u{00E4}ndert:       mit-commit-message-lints/src/lints/lib/missing_pivotal_tracker_id.rs
            #
            # ------------------------ >8 ------------------------
            # \u{00C4}ndern oder entfernen Sie nicht die obige Zeile.
            # Alles unterhalb von ihr wird ignoriert.
            diff --git a/mit-commit-message-lints/src/lints/lib/missing_pivotal_tracker_id.rs b/mit-commit-message-lints/src/lints/lib/missing_pivotal_tracker_id.rs
            index 5a83784..ebaee48 100644
            --- a/mit-commit-message-lints/src/lints/lib/missing_pivotal_tracker_id.rs
            +++ b/mit-commit-message-lints/src/lints/lib/missing_pivotal_tracker_id.rs
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


            "
        ), &None);
    }

    #[test]
    fn single_line() {
        test_subject_not_seperate_from_body("An example commit", &None);
    }

    #[test]
    fn gutter_missing() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                An example commit
                This is an example commit
                "
            ),
            &Some(Problem::new(
                HELP_MESSAGE.into(),
                Code::SubjectNotSeparateFromBody,
            )),
        );
        test_subject_not_seperate_from_body(
            indoc!(
                "
                An example commit
                This is an example commit
                It has more lines
                It has even more lines
                "
            ),
            &Some(Problem::new(
                HELP_MESSAGE.into(),
                Code::SubjectNotSeparateFromBody,
            )),
        );
    }

    fn test_subject_not_seperate_from_body(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::new(message.into()));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
