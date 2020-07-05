use mit_commit::CommitMessage;

use crate::lints::lib::problem::Code;
use crate::lints::lib::Problem;

pub(crate) const CONFIG: &str = "body-wider-than-72-characters";

const HELP_MESSAGE: &str = "Please keep the width of the body 72 characters or under";
const ERROR: &str = "Your commit message is not well formed";

fn has_problem(commit: &CommitMessage) -> bool {
    commit
        .get_body()
        .iter()
        .flat_map(|body| {
            String::from(body.clone())
                .lines()
                .map(String::from)
                .collect::<Vec<String>>()
        })
        .any(|line| line.len() > 72)
}

pub(crate) fn lint(commit: &CommitMessage) -> Option<Problem> {
    if has_problem(commit) {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::BodyWiderThan72Characters,
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
    fn narrower_than_72_characters() {
        test_body_wider_than_72_characters(&format!("Subject\n\n{}", "x".repeat(72)), &None);
    }

    #[test]
    fn no_body() {
        test_body_wider_than_72_characters("Subject", &None);
    }

    #[test]
    fn body_ok_but_comments_longer_than_72() {
        let message = indoc!(
            "
            Remove duplicated function

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
        );
        test_body_wider_than_72_characters(&format!("{}\n\n{}", "x".repeat(72), message), &None);
    }

    #[test]
    fn longer_than_72_characters() {
        test_body_wider_than_72_characters(
            &format!("Subject\n\n{}", "x".repeat(73)),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::BodyWiderThan72Characters,
            )),
        );
    }

    #[test]
    fn first_line_ok_but_second_line_too_long() {
        test_body_wider_than_72_characters(
            &format!("Subject\n\nx\n{}\nx\n", "x".repeat(73)),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::BodyWiderThan72Characters,
            )),
        );
    }

    fn test_body_wider_than_72_characters(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
