use crate::lints::lib::problem::Code;
use crate::lints::lib::{CommitMessage, Problem};
use indoc::indoc;

pub(crate) const CONFIG: &str = "subject-not-separated-from-body";

const HELP_MESSAGE: &str = indoc!(
    "
    Your commit is not well formed

    To fix this separate subject from body with a blank line

    For example:

    Aligns time sprondle

    The time sprondle is drifting backwards in the current
    configuration, this corrects that.
    "
);

const SIZE_OF_SUBJECT: usize = 1;

fn has_subject_not_separate_from_body(commit_message: &CommitMessage) -> bool {
    let line_count = commit_message.content_line_count();

    match line_count {
        1 => false,
        2 => true,
        _ => commit_message.get_body().lines().count() + SIZE_OF_SUBJECT == line_count,
    }
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_subject_not_separate_from_body(commit_message) {
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
                indoc!(
                    "
                    Your commit is not well formed

                    To fix this separate subject from body with a blank line

                    For example:

                    Aligns time sprondle

                    The time sprondle is drifting backwards in the current
                    configuration, this corrects that.
                    "
                )
                .into(),
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
                indoc!(
                    "
                    Your commit is not well formed

                    To fix this separate subject from body with a blank line

                    For example:

                    Aligns time sprondle

                    The time sprondle is drifting backwards in the current
                    configuration, this corrects that.
                    "
                )
                .into(),
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
