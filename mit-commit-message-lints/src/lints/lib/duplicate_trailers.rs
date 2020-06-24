use std::{collections::HashSet, iter::FromIterator};

use crate::lints::lib::problem::Code;
use crate::lints::lib::{CommitMessage, Problem, Trailer};

pub(crate) const CONFIG: &str = "duplicated-trailers";

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 2] = ["Signed-off-by", "Co-authored-by"];
const FIELD_SINGULAR: &str = "field";
const FIELD_PLURAL: &str = "fields";

fn get_duplicated_trailers(commit_message: &CommitMessage) -> Vec<String> {
    TRAILERS_TO_CHECK_FOR_DUPLICATES
        .iter()
        .filter_map(|trailer| filter_without_duplicates(commit_message, trailer))
        .collect::<Vec<String>>()
}

fn filter_without_duplicates(commit_message: &CommitMessage, trailer_key: &str) -> Option<String> {
    Some(trailer_key)
        .map(String::from)
        .filter(|trailer| has_duplicated_trailer(commit_message, trailer))
}

fn has_duplicated_trailer(commit_message: &CommitMessage, trailer_key: &str) -> bool {
    Some(commit_message.get_trailer(trailer_key))
        .map(|trailers| (trailers.clone(), trailers))
        .map(|(commit, unique)| (commit, HashSet::<Trailer>::from_iter(unique)))
        .map(|(commit, unique)| commit.len() != unique.len())
        .unwrap()
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    let duplicated_trailers = get_duplicated_trailers(commit_message);
    if duplicated_trailers.is_empty() {
        None
    } else {
        let warning = format!(
            "Your commit message has duplicated trailers\n\nYou can fix this by deleting the duplicated \"{}\" {}",
            duplicated_trailers.join("\", \""),
            if duplicated_trailers.len() > 1 {
                FIELD_PLURAL
            } else {
                FIELD_SINGULAR
            }
        );
        Some(Problem::new(warning, Code::DuplicatedTrailers))
    }
}

#[cfg(test)]
mod tests_has_duplicated_trailers {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn duplicated_trailers() {
        test_lint_duplicated_trailers(
            indoc!(
                "
                An example commit

                This is an example commit without any duplicate trailers
                "
            )
            .into(),
            &None,
        );
        test_lint_duplicated_trailers(
            indoc!(
                "
                An example commit

                This is an example commit without any duplicate trailers

                Signed-off-by: Billie Thompson <email@example.com>
                Signed-off-by: Billie Thompson <email@example.com>
                Co-authored-by: Billie Thompson <email@example.com>
                Co-authored-by: Billie Thompson <email@example.com>
                "
            )
            .into(),
            &Some(Problem::new(
                "Your commit message has duplicated trailers\n\nYou can fix this by deleting the duplicated \"Signed-off-by\", \"Co-authored-by\" fields".into(),
                Code::DuplicatedTrailers,
            )),
        );
        test_lint_duplicated_trailers(
            indoc!(
                "
                An example commit

                This is an example commit without any duplicate trailers

                Signed-off-by: Billie Thompson <email@example.com>
                Signed-off-by: Billie Thompson <email@example.com>
                "
            )
            .into(),
            &Some(Problem::new(
                "Your commit message has duplicated trailers\n\nYou can fix this by deleting the duplicated \"Signed-off-by\" field".into(),
                Code::DuplicatedTrailers,
            )),
        );
        test_lint_duplicated_trailers(
            indoc!(
                "
                An example commit

                This is an example commit without any duplicate trailers

                Co-authored-by: Billie Thompson <email@example.com>
                Co-authored-by: Billie Thompson <email@example.com>
                "
            )
            .into(),
            &Some(Problem::new(
                "Your commit message has duplicated trailers\n\nYou can fix this by deleting the duplicated \"Co-authored-by\" field".into(),
                Code::DuplicatedTrailers,
            )),
        );
    }

    fn test_lint_duplicated_trailers(message: String, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::new(message));
        assert_eq!(
            actual, expected,
            "Expected {:?}, found {:?}",
            expected, actual
        );
    }
}

#[cfg(test)]
mod tests_has_duplicated_trailer {
    use crate::lints::lib::duplicate_trailers::has_duplicated_trailer;
    use crate::lints::CommitMessage;
    use indoc::indoc;

    fn test_has_duplicated_trailer(message: &str, trailer: &str, expected: bool) {
        let actual = has_duplicated_trailer(&CommitMessage::new(message.into()), trailer);
        assert_eq!(
            actual, expected,
            "Message {:?} with trailer {:?} should have returned {:?}, found {:?}",
            message, trailer, expected, actual
        );
    }

    #[test]
    fn no_trailer() {
        test_has_duplicated_trailer(
            indoc!(
                "
                An example commit

                This is an example commit without any duplicate trailers
                "
            ),
            "Signed-off-by",
            false,
        );
    }

    #[test]
    fn duplicated_trailer() {
        test_has_duplicated_trailer(
            indoc!(
                "
                An example commit

                This is an example commit with duplicate trailers

                Signed-off-by: Billie Thompson <email@example.com>
                Signed-off-by: Billie Thompson <email@example.com>
                "
            ),
            "Signed-off-by",
            true,
        );
    }

    #[test]
    fn two_trailers_but_no_duplicates() {
        test_has_duplicated_trailer(
            indoc!(
                "
                An example commit

                This is an example commit without any duplicate trailers

                Signed-off-by: Billie Thompson <billie@example.com>
                Signed-off-by: Ada Lovelace <ada@example.com>
                "
            ),
            "Signed-off-by",
            false,
        );
    }

    #[test]
    fn one_trailer() {
        test_has_duplicated_trailer(
            indoc!(
                "
                An example commit

                This is an example commit without any duplicate trailers

                Signed-off-by: Billie Thompson <email@example.com>
                "
            ),
            "Signed-off-by",
            false,
        );
    }

    #[test]
    fn missing_colon_in_trailer() {
        test_has_duplicated_trailer(
            indoc!(
                "
                An example commit

                This is an example commit without any duplicate trailers

                Signed-off-by Billie Thompson <email@example.com>
                Signed-off-by Billie Thompson <email@example.com>
                "
            ),
            "Signed-off-by",
            false,
        );
    }

    #[test]
    fn customised_trailer() {
        test_has_duplicated_trailer(
            indoc!(
                "
                An example commit

                This is an example commit with duplicate trailers

                Anything: Billie Thompson <email@example.com>
                Anything: Billie Thompson <email@example.com>
                "
            ),
            "Anything",
            true,
        );
    }
}
