use indoc::indoc;
use mit_commit::CommitMessage;
use regex::Regex;

use crate::lints::lib::problem::Code;
use crate::lints::lib::Problem;

pub(crate) const CONFIG: &str = "not-conventional-commit";

const HELP_MESSAGE: &str = indoc!(
    "
    You can fix it by following style

    <type>[optional scope]: <description>

    [optional body]

    [optional footer(s)]

    You can read more at https://www.conventionalcommits.org/"
);

const ERROR: &str = "Your commit message isn't conventional";

lazy_static! {
    static ref RE: Regex = Regex::new("^[^()\\s]+(\\(\\w+\\))?!?: ").unwrap();
}

fn has_problem(commit_message: &CommitMessage) -> bool {
    let subject: String = commit_message.get_subject().into();

    !RE.is_match(&subject)
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_problem(&commit_message) {
        Some(Problem::new(
            ERROR.into(),
            HELP_MESSAGE.into(),
            Code::NotConventionalCommit,
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

    // Examples from https://www.conventionalcommits.org/en/v1.0.0/

    #[test]
    fn commit_message_with_description_and_breaking_change_footer() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                feat: allow provided config object to extend other configs

                BREAKING CHANGE: `extends` key in config file is now used for extending other config files
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_bang_to_draw_attention_to_breaking_change() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                refactor!: drop support for Node 6
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_both_bang_and_breaking_change_footer() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                refactor!: drop support for Node 6

                BREAKING CHANGE: refactor to use JavaScript features not available in Node 6.
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_no_body() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                docs: correct spelling of CHANGELOG
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_scope() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                feat(lang): add polish language
                "
            ),
            &None,
        );
    }

    #[test]
    fn commit_message_with_multi_paragraph_body_and_multiple_footers() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                fix: correct minor typos in code

                see the issue for details

                on typos fixed.

                Reviewed-by: Z
                Refs #133
                "
            ),
            &None,
        );
    }

    #[test]
    fn revert_example() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                revert: let us never again speak of the noodle incident

                Refs: 676104e, a215868
                "
            ),
            &None,
        );
    }

    #[test]
    fn non_conventional() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
            )),
        );
    }

    #[test]
    fn missing_bracket() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                fix(example: An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
            )),
        );
    }

    #[test]
    fn missing_space() {
        test_subject_not_seperate_from_body(
            indoc!(
                "
                fix(example):An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
            )),
        );
    }

    fn test_subject_not_seperate_from_body(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
