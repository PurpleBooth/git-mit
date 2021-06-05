use mit_commit::CommitMessage;

use crate::console::exit::Code;
use crate::lints::lib::Problem;

pub(crate) const CONFIG: &str = "subject-line-not-capitalized";

const HELP_MESSAGE: &str = "The subject line is a title, and as such should be capitalised.\n\nYou can fix this by capitalising the first character in the subject";
const ERROR: &str = "Your commit message is missing a capital letter";

fn has_problem(commit_message: &CommitMessage) -> bool {
    commit_message
        .get_subject()
        .chars()
        .next()
        .map_or(false, |character| {
            character.to_string() != character.to_uppercase().to_string()
        })
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(commit_message) {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::SubjectNotCapitalized,
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
    fn capitalised() {
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
    fn lower_case() {
        run_test(
            indoc!(
                "
                subject line
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::SubjectNotCapitalized,
            )),
        );
    }

    #[test]
    fn numbers_are_fine() {
        run_test(
            indoc!(
                "
                1234567
                "
            ),
            &None,
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
