use std::{collections::HashSet, iter::FromIterator};

use crate::lints::{CommitMessage, LintCode, LintProblem};

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 2] = ["Signed-off-by", "Co-authored-by"];
const FIELD_SINGULAR: &str = "field";
const FIELD_PLURAL: &str = "fields";

fn has_duplicated_trailers(commit_message: &CommitMessage) -> Vec<String> {
    TRAILERS_TO_CHECK_FOR_DUPLICATES
        .iter()
        .filter_map(|trailer| filter_without_duplicates(commit_message, trailer))
        .collect::<Vec<String>>()
}

fn filter_without_duplicates(commit_message: &CommitMessage, trailer: &str) -> Option<String> {
    Some(trailer)
        .map(String::from)
        .filter(|trailer| has_duplicated_trailer(commit_message, trailer))
}

fn has_duplicated_trailer(commit_message: &CommitMessage, trailer: &str) -> bool {
    Some(commit_message.get_trailer(trailer))
        .map(|trailers| (trailers.clone(), trailers.clone()))
        .map(|(commit, unique)| (commit, HashSet::<&str>::from_iter(unique)))
        .map(|(commit, unique)| commit.len() != unique.len())
        .unwrap()
}

pub(crate) fn lint_duplicated_trailers(commit_message: &str) -> Option<LintProblem> {
    let duplicated_trailers = has_duplicated_trailers(&CommitMessage::new(commit_message));
    if duplicated_trailers.is_empty() {
        None
    } else {
        let mut fields = FIELD_SINGULAR;
        if duplicated_trailers.len() > 1 {
            fields = FIELD_PLURAL
        }

        Some(LintProblem::new(
            format!(
                r#"Your commit cannot have the same name duplicated in the "{}" {}

You can fix this by removing the duplicated field when you commit again
"#,
                duplicated_trailers.join("\", \""),
                fields
            ),
            LintCode::DuplicatedTrailers,
        ))
    }
}

#[cfg(test)]
mod tests_has_duplicated_trailers {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn duplicated_trailers() {
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers
"#,
            &[],
        );
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#,
            &["Signed-off-by".into(), "Co-authored-by".into()],
        );
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#,
            &["Signed-off-by".into()],
        );
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#,
            &["Co-authored-by".into()],
        );
    }

    fn test_has_duplicated_trailers(message: &str, expected: &[String]) {
        let actual = has_duplicated_trailers(&CommitMessage::new(message));
        assert_eq!(
            actual, expected,
            "Expected {:?}, found {:?}",
            expected, actual
        );
    }

    #[cfg(test)]
    mod tests_has_duplicated_trailer {
        use crate::lints::{duplicate_trailers::has_duplicated_trailer, CommitMessage};

        fn test_has_duplicated_trailer(message: &str, trailer: &str, expected: bool) {
            let actual = has_duplicated_trailer(&CommitMessage::new(message), trailer);
            assert_eq!(
                actual, expected,
                "Message {:?} with trailer {:?} should have returned {:?}, found {:?}",
                message, trailer, expected, actual
            );
        }

        #[test]
        fn no_trailer() {
            test_has_duplicated_trailer(
                r#"
An example commit

This is an example commit without any duplicate trailers
"#,
                "Signed-off-by",
                false,
            );
        }

        #[test]
        fn duplicated_trailer() {
            test_has_duplicated_trailer(
                r#"
An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#,
                "Signed-off-by",
                true,
            );
        }

        #[test]
        fn two_trailers_but_no_duplicates() {
            test_has_duplicated_trailer(
                r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <billie@example.com>
Signed-off-by: Ada Lovelace <ada@example.com>
"#,
                "Signed-off-by",
                false,
            );
        }

        #[test]
        fn one_trailer() {
            test_has_duplicated_trailer(
                r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
"#,
                "Signed-off-by",
                false,
            );
        }

        #[test]
        fn missing_colon_in_trailer() {
            test_has_duplicated_trailer(
                r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by Billie Thompson <email@example.com>
Signed-off-by Billie Thompson <email@example.com>
"#,
                "Signed-off-by",
                false,
            );
        }

        #[test]
        fn customised_trailer() {
            test_has_duplicated_trailer(
                r#"
An example commit

This is an example commit with duplicate trailers

Anything: Billie Thompson <email@example.com>
Anything: Billie Thompson <email@example.com>
"#,
                "Anything",
                true,
            );
        }
    }
}
