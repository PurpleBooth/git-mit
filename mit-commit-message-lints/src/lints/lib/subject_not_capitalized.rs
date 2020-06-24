use crate::lints::lib::problem::Code;
use crate::lints::lib::{CommitMessage, Problem};
use indoc::indoc;

pub(crate) const CONFIG: &str = "subject-line-not-capitalized";

const HELP_MESSAGE: &str = indoc!(
    "
    Your commit message is missing a capital letter

    You can fix this by capitalising the first character in the subject
    "
);

fn has_problem(commit_message: &CommitMessage) -> bool {
    if let Some(character) = commit_message.get_subject().chars().next() {
        return character.to_string() != character.to_uppercase().to_string();
    }

    false
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(commit_message) {
        Some(Problem::new(
            HELP_MESSAGE.into(),
            Code::SubjectLintNotCapitalized,
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
                HELP_MESSAGE.into(),
                Code::SubjectLintNotCapitalized,
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
        let actual = &lint(&CommitMessage::new(message.into()));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
