use indoc::indoc;
use mit_commit::CommitMessage;

use crate::{console::exit::Code, lints::lib::Problem};

pub(crate) const CONFIG: &str = "not-emoji-log";

const HELP_MESSAGE: &str = indoc!(
    "
    It's important to follow the emoji log style when creating your commit message. By using this \
    style we can automatically generate changelogs.

    You can fix it using one of the prefixes:

    \u{1f4e6} NEW:
    \u{1f44c} IMPROVE:
    \u{1f41b} FIX:
    \u{1f4d6} DOC:
    \u{1f680} RELEASE:
    \u{1f916} TEST:
    \u{203c}\u{fe0f} BREAKING:

    You can read more at https://github.com/ahmadawais/Emoji-Log
    "
);

const ERROR: &str = "Your commit message isn't in emoji log style";

const PREFIXES: &[&str] = &[
    "\u{1f4e6} NEW: ",
    "\u{1f44c} IMPROVE: ",
    "\u{1f41b} FIX: ",
    "\u{1f4d6} DOC: ",
    "\u{1f680} RELEASE: ",
    "\u{1f916} TEST: ",
    "\u{203c}\u{fe0f} BREAKING: ",
];

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if PREFIXES
        .iter()
        .any(|x| commit_message.get_subject().to_string().starts_with(x))
    {
        None
    } else {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotEmojiLog,
        ))
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;

    use super::*;

    #[test]
    fn new() {
        run_lint(
            indoc!(
                "
                \u{1f4e6} NEW: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn improve() {
        run_lint(
            indoc!(
                "
                \u{1f44c} IMPROVE: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn fix() {
        run_lint(
            indoc!(
                "
                \u{1f41b} FIX: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn docs() {
        run_lint(
            indoc!(
                "
                \u{1f4d6} DOC: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn release() {
        run_lint(
            indoc!(
                "
                \u{1f680} RELEASE: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn test() {
        run_lint(
            indoc!(
                "
                \u{1f916} TEST: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn bc() {
        run_lint(
            indoc!(
                "
                \u{203c}\u{fe0f} BREAKING: An example commit

                This is an example commit
                "
            ),
            &None,
        );
    }

    #[test]
    fn no_gap() {
        run_lint(
            indoc!(
                "
                \u{203c}\u{fe0f} BREAKING:An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotEmojiLog,
            )),
        );
    }

    #[test]
    fn unknown_emoji() {
        run_lint(
            indoc!(
                "
                \u{1f408} UNKNOWN: An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotEmojiLog,
            )),
        );
    }

    #[test]
    fn not_emoji() {
        run_lint(
            indoc!(
                "
                An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotEmojiLog,
            )),
        );
    }

    fn run_lint(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
