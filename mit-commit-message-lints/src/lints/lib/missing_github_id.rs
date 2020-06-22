use regex::Regex;

use crate::lints::lib::problem::Code;
use crate::lints::lib::{CommitMessage, Problem};
use indoc::indoc;

pub(crate) const CONFIG: &str = "github-id-missing";

const GITHUB_HELP_MESSAGE: &str = indoc!(
    "
    Your commit is missing a GitHub ID

    You can fix this by adding a key like the following examples

    close #642
    closes: #642
    Closed GH-642
    fix #642
    This fixes #642
    fixed #642
    resolve #642
    resolves #642
    resolved #642
    Issue #642

    GitHub also supports these alternative styles of referring to IDs

    GH-642
    AnUser/git-mit#642
    AnOrganisation/git-mit#642

    Be careful just putting '#123' on a line by itself, as '#' is the default comment indicator
    "
);

const REGEX_GITHUB_ID_REGEX: &str =
    r"(?m)(^| )([a-zA-Z0-9_-]{3,39}/[a-zA-Z0-9-]{1,}#|GH-|#)[0-9]{1,}( |$)";

fn has_missing_github_id(commit_message: &CommitMessage) -> bool {
    let re = Regex::new(REGEX_GITHUB_ID_REGEX).unwrap();
    !commit_message.matches_pattern(&re)
}

pub(crate) fn lint(commit_message: &CommitMessage) -> Option<Problem> {
    if has_missing_github_id(commit_message) {
        Some(Problem::new(
            GITHUB_HELP_MESSAGE.into(),
            Code::GitHubIdMissing,
        ))
    } else {
        None
    }
}

#[cfg(test)]
mod tests_has_missing_github_id {
    #![allow(clippy::wildcard_imports)]

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn id_and_close() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                close #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                closes: #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                Closed GH-642
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_and_fix() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                fix #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                This fixes #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                fixed #642
                "
            ),
            &None,
        );
    }
    #[test]
    fn id_and_resolve() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                resolve #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                resolves #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                resolved #642
                "
            ),
            &None,
        );
    }
    #[test]
    fn issue() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                Issue #642
                "
            ),
            &None,
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                Issue #642
                "
            ),
            &None,
        );
    }
    #[test]
    fn gh_id_variant() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                GH-642
                "
            ),
            &None,
        );
    }
    #[test]
    fn hash_alone_variant() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                #642
                "
            ),
            &None,
        );
    }

    #[test]
    fn long_variant() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                AnUser/git-mit#642
                "
            ),
            &None,
        );

        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                AnOrganisation/git-mit#642
                "
            ),
            &None,
        );
    }

    #[test]
    fn id_missing() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit
                "
            ),
            &Some(Problem::new(
                indoc!(
                    "
                    Your commit is missing a GitHub ID

                    You can fix this by adding a key like the following examples

                    close #642
                    closes: #642
                    Closed GH-642
                    fix #642
                    This fixes #642
                    fixed #642
                    resolve #642
                    resolves #642
                    resolved #642
                    Issue #642

                    GitHub also supports these alternative styles of referring to IDs

                    GH-642
                    AnUser/git-mit#642
                    AnOrganisation/git-mit#642

                    Be careful just putting '#123' on a line by itself, as '#' is the default comment indicator
                    "
                ).into(),
                Code::GitHubIdMissing,
            )),
        );
    }
    #[test]
    fn id_malformed() {
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                H-123
                "
            ),
            &Some(Problem::new(
                indoc!(
                    "
                    Your commit is missing a GitHub ID

                    You can fix this by adding a key like the following examples

                    close #642
                    closes: #642
                    Closed GH-642
                    fix #642
                    This fixes #642
                    fixed #642
                    resolve #642
                    resolves #642
                    resolved #642
                    Issue #642

                    GitHub also supports these alternative styles of referring to IDs

                    GH-642
                    AnUser/git-mit#642
                    AnOrganisation/git-mit#642

                    Be careful just putting '#123' on a line by itself, as '#' is the default comment indicator
                    "
                )
                .into(),
                Code::GitHubIdMissing,
            )),
        );
        test_has_missing_github_id(
            indoc!(
                "
                An example commit

                This is an example commit

                git-mit#123
                "
            ),
            &Some(Problem::new(
                indoc!(
                    "
                    Your commit is missing a GitHub ID

                    You can fix this by adding a key like the following examples

                    close #642
                    closes: #642
                    Closed GH-642
                    fix #642
                    This fixes #642
                    fixed #642
                    resolve #642
                    resolves #642
                    resolved #642
                    Issue #642

                    GitHub also supports these alternative styles of referring to IDs

                    GH-642
                    AnUser/git-mit#642
                    AnOrganisation/git-mit#642

                    Be careful just putting '#123' on a line by itself, as '#' is the default comment indicator
                    "
                )
                .into(),
                Code::GitHubIdMissing,
            )),
        );
    }

    fn test_has_missing_github_id(message: &str, expected: &Option<Problem>) {
        let actual = &lint(&CommitMessage::new(message.into()));
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }
}
