use std::collections::BTreeMap;
use std::ops::Add;

use mit_commit::CommitMessage;
use mit_commit::Trailer as NgTrailer;

use crate::lints::lib::Code;
use crate::lints::lib::Problem;

pub(crate) const CONFIG: &str = "duplicated-trailers";

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 2] = ["Signed-off-by", "Co-authored-by"];
const FIELD_SINGULAR: &str = "field";
const ERROR: &str = "Your commit message has duplicated trailers";

const FIELD_PLURAL: &str = "fields";

fn get_duplicated_trailers(commit_message: &CommitMessage) -> Vec<String> {
    commit_message
        .get_trailers()
        .iter()
        .fold(
            BTreeMap::new(),
            |acc: BTreeMap<&NgTrailer, usize>, trailer| {
                let mut next = acc.clone();
                match acc.get(trailer) {
                    Some(count) => next.insert(trailer, count.add(1)),
                    None => next.insert(trailer, 1),
                };

                next
            },
        )
        .into_iter()
        .filter_map(|(trailer, usize)| {
            let key: &str = &trailer.get_key();

            if usize > 1 && TRAILERS_TO_CHECK_FOR_DUPLICATES.contains(&key) {
                Some(trailer.get_key())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub(crate) fn lint(commit: &CommitMessage) -> Option<Problem> {
    let duplicated_trailers = get_duplicated_trailers(commit);
    if duplicated_trailers.is_empty() {
        None
    } else {
        let warning = warning(&duplicated_trailers);
        Some(Problem::new(
            ERROR.into(),
            warning,
            Code::DuplicatedTrailers,
        ))
    }
}

fn warning(duplicated_trailers: &[String]) -> String {
    let warning = format!(
        "You can fix this by deleting the duplicated \"{}\" {}",
        duplicated_trailers.join("\", \""),
        if duplicated_trailers.len() > 1 {
            FIELD_PLURAL
        } else {
            FIELD_SINGULAR
        }
    );
    warning
}

#[cfg(test)]
mod tests_has_duplicated_trailers {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;
    use mit_commit::CommitMessage;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn commit_without_trailers() {
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
    }

    #[test]
    fn two_duplicates() {
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
            ).into(),
            &Some(Problem::new(ERROR.into(),
                               "You can fix this by deleting the duplicated \"Co-authored-by\", \"Signed-off-by\" fields".into(),
                               Code::DuplicatedTrailers,
            )),
        );
    }

    #[test]
    fn signed_off_by() {
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
                ERROR.into(),
                "You can fix this by deleting the duplicated \"Signed-off-by\" field".into(),
                Code::DuplicatedTrailers,
            )),
        );
    }

    #[test]
    fn co_authored_by() {
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
                ERROR.into(),
                "You can fix this by deleting the duplicated \"Co-authored-by\" field".into(),
                Code::DuplicatedTrailers,
            )),
        );
    }

    #[test]
    fn trailer_like_duplicates_in_the_scissors_section() {
        test_lint_duplicated_trailers(
            indoc!(
                "
                Move to specdown
                # Bitte geben Sie eine Commit-Beschreibung fur Ihre Anderungen ein. Zeilen,
                # die mit '#' beginnen, werden ignoriert, und eine leere Beschreibung

                # ------------------------ >8 ------------------------
                # Andern oder entfernen Sie nicht die obige Zeile.
                # Alles unterhalb von ihr wird ignoriert.
                diff --git a/Makefile b/Makefile
                index 0d3fc98..38a2784 100644
                --- a/Makefile
                +++ b/Makefile
                +
                 This is a commit message that has trailers and is invalid

                -Signed-off-by: Someone Else <someone@example.com>
                -Signed-off-by: Someone Else <someone@example.com>
                 Co-authored-by: Billie Thompson <billie@example.com>
                 Co-authored-by: Billie Thompson <billie@example.com>
                +Signed-off-by: Someone Else <someone@example.com>
                +Signed-off-by: Someone Else <someone@example.com>


                 ---
                @@ -82,6 +82,7 @@ Co-authored-by: Billie Thompson <billie@example.com>
                 Your commit message has duplicated trailers

                 You can fix this by deleting the duplicated \"Signed-off-by\", \"Co-authored-by\" fields
                +
                "
            ).into(),
            &None,
        );
    }

    #[test]
    fn other_trailers() {
        test_lint_duplicated_trailers(
            indoc!(
                "
                An example commit

                This is an example commit without any duplicate trailers

                Anything: Billie Thompson <email@example.com>
                Anything: Billie Thompson <email@example.com>
                "
            )
            .into(),
            &None,
        );
    }

    fn test_lint_duplicated_trailers(message: String, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual, expected,
            "Expected {:?}, found {:?}",
            expected, actual
        );
    }
}
