use mit_commit::CommitMessage;

use crate::{console::exit::Code, lints::lib::Problem};

pub(crate) const CONFIG: &str = "subject-line-ends-with-period";

const ERROR: &str = "Your commit message ends with a period";
const HELP_MESSAGE: &str = "It's important to keep your commits short, because we only have a \
limited number of characters to use (72) before the subject line is truncated. Full stops aren't \
normally in subject lines, and take up an extra character, so we shouldn't use them in commit \
message subjects.\n\nYou can fix this by removing the period";

fn has_problem(commit_message: &CommitMessage) -> bool {
    matches!(commit_message.get_subject().chars().rev().next(), Some('.'))
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(commit_message) {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectEndsWithPeriod,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;

    use super::*;

    #[test]
    fn subject_does_not_end_with_period() {
        run_test(
            indoc!(
                "
                Subject Line
                "
            ),
            &None,
        );
    }

    #[test]
    fn subject_ends_with_period() {
        run_test(
            indoc!(
                "
                Subject Line.
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
            )),
        );
    }

    #[test]
    fn subject_has_period_then_whitespace() {
        run_test(
            indoc!(
                "
                Subject Line.
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectEndsWithPeriod,
            )),
        );
    }

    fn run_test(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
