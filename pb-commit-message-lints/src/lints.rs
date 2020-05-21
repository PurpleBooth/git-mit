use crate::{
    lints::Lints::{DuplicatedTrailers, PivotalTrackerIdMissing},
    VcsConfig,
};
use regex::Regex;
use std::{collections::HashSet, error::Error};

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 2] = ["Signed-off-by", "Co-authored-by"];
const CONFIG_DUPLICATED_TRAILERS: &str = "pb.lint.duplicated-trailers";
const CONFIG_PIVOTAL_TRACKER_ID_MISSING: &str = "pb.lint.pivotal-tracker-id-missing";
const REGEX_PIVOTAL_TRACKER_ID: &str =
    r"\[(((finish|fix)(ed|es)?|complete[ds]?|deliver(s|ed)?) )?#\d+([, ]#\d+)*]";

/// The lints that are supported
#[derive(Debug, Eq, PartialEq)]
pub enum Lints {
    DuplicatedTrailers,
    PivotalTrackerIdMissing,
}

use std::iter::FromIterator;

/// Look at a git config and work out what lints should be turned on and off
///
/// # Example
///
/// ```
/// use git2::Repository;
/// use pb_commit_message_lints::{
///     get_lint_configuration,
///     Git2VcsConfig,
///     Lints::DuplicatedTrailers,
/// };
/// use tempfile::TempDir;
///
/// let config = TempDir::new()
///     .map(TempDir::into_path)
///     .map(|x| x.join("repository"))
///     .map(Repository::init)
///     .expect("Failed to initialise the repository")
///     .expect("Failed create temporary directory")
///     .config()
///     .map(Git2VcsConfig::new)
///     .expect("Failed to get configuration");
///
/// assert_eq!(
///     get_lint_configuration(&config).expect("To be able to get a configuration"),
///     vec![DuplicatedTrailers],
/// )
/// ```
///
/// # Errors
///
/// Will return `Err` if we can't read the git configuration for some reason or it's not parsable
pub fn get_lint_configuration(config: &dyn VcsConfig) -> Result<Vec<Lints>, Box<dyn Error>> {
    let mut result_vec: Vec<Lints> = vec![];

    match config.get_bool(CONFIG_DUPLICATED_TRAILERS) {
        Some(false) => {}
        _ => result_vec.push(DuplicatedTrailers),
    }

    if let Some(true) = config.get_bool(CONFIG_PIVOTAL_TRACKER_ID_MISSING) {
        result_vec.push(PivotalTrackerIdMissing)
    }

    Ok(result_vec)
}

/// Check if a commit message message has duplicated trailers with names in
///
/// # Example
///
/// ```
/// use pb_commit_message_lints::has_duplicated_trailers;
///
/// assert_eq!(
///     has_duplicated_trailers(
///         r#"
/// An example commit
///
/// This is an example commit without any duplicate trailers
///
/// Signed-off-by: Billie Thompson <email@example.com>
/// Signed-off-by: Billie Thompson <email@example.com>
/// "#
///     ),
///     Some(vec!["Signed-off-by".into()])
/// );
///
/// assert_eq!(
///     has_duplicated_trailers(
///         r#"
/// An example commit
///
/// This is an example commit without any duplicate trailers
///
/// Co-authored-by: Billie Thompson <email@example.com>
/// Co-authored-by: Billie Thompson <email@example.com>
/// "#
///     ),
///     Some(vec!["Co-authored-by".into()])
/// );
/// ```
#[must_use]
pub fn has_duplicated_trailers(commit_message: &str) -> Option<Vec<String>> {
    let trailer_duplications = |x: &&str| {
        Some(*x)
            .map(String::from)
            .filter(|x| has_duplicated_trailer(commit_message, x))
    };
    let is_not_empty = |x: &Vec<_>| !x.is_empty();
    Some(
        TRAILERS_TO_CHECK_FOR_DUPLICATES
            .iter()
            .filter_map(trailer_duplications)
            .collect::<Vec<String>>(),
    )
    .filter(is_not_empty)
}

/// Check if a commit message message has a pivotal tracker id in it
///
/// # Example
///
/// ```
/// use pb_commit_message_lints::has_missing_pivotal_tracker_id;
///
/// assert_eq!(
///     has_missing_pivotal_tracker_id(
///         r#"
/// [fix #12345678] correct bug the build
/// "#,
///     ),
///     None
/// );
/// assert_eq!(
///     has_missing_pivotal_tracker_id(
///         r#"
/// Add a new commit linter
///
/// This will add a new linter. This linter checks for the presence of a Pivotal Tracker Id. In this
/// example I have forgotten my Id.
/// "#,
///     ),
///     Some(())
/// );
///
/// assert_eq!(
///     has_missing_pivotal_tracker_id(
///         r#"
/// Add a new commit linter
///
/// This will add a new linter. This linter checks for the presence of a Pivotal Tracker Id. In this
/// example I have remembered my Id
///
/// [deliver #12345678,#88335556,#87654321]
/// "#
///     ),
///     None
/// );
/// ```
#[must_use]
pub fn has_missing_pivotal_tracker_id(commit_message: &str) -> Option<()> {
    let re = Regex::new(REGEX_PIVOTAL_TRACKER_ID).unwrap();
    let to_empty_some = |_| Some(());
    let is_not_match = |x: &String| !re.is_match(x);
    Some(commit_message)
        .map(str::to_lowercase)
        .filter(is_not_match)
        .and_then(to_empty_some)
}

fn has_duplicated_trailer(commit_message: &str, trailer: &str) -> bool {
    let starts_with_trailer = |x: &&str| x.starts_with(&format!("{}:", trailer));
    let trailers: Vec<&str> = commit_message.lines().filter(starts_with_trailer).collect();

    let unique_trailers: std::collections::HashSet<&str> =
        HashSet::from_iter(trailers.clone().into_iter());

    trailers.len() != unique_trailers.len()
}

#[cfg(test)]
mod tests_has_duplicated_trailers {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn has_duplicated_trailers_runs_both_co_authored_and_signed_off_by() {
        assert_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers
"#,
            &None,
        );
        assert_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#,
            &Some(vec!["Signed-off-by".into(), "Co-authored-by".into()]),
        );
        assert_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#,
            &Some(vec!["Signed-off-by".into()]),
        );
        assert_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#,
            &Some(vec!["Co-authored-by".into()]),
        );
    }

    fn assert_has_duplicated_trailers(message: &str, expected: &Option<Vec<String>>) {
        let actual = has_duplicated_trailers(message);
        assert_eq!(
            actual, *expected,
            "Expected {:?}, found {:?}",
            expected, actual
        );
    }

    fn assert_has_duplicated_trailer(message: &str, trailer: &str, expected: bool) {
        let actual = has_duplicated_trailer(message, trailer);
        assert_eq!(
            actual, expected,
            "Message {:?} with trailer {:?} should have returned {:?}, found {:?}",
            message, trailer, expected, actual
        );
    }

    #[test]
    fn has_duplicated_trailer_does_nothing_on_no_trailer() {
        assert_has_duplicated_trailer(
            r#"
An example commit

This is an example commit without any duplicate trailers
"#,
            "Signed-off-by",
            false,
        );
    }

    #[test]
    fn has_duplicated_trailer_detects_duplicated_trailers() {
        assert_has_duplicated_trailer(
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
    fn has_duplicated_trailer_two_trailers_with_different_names_is_fine() {
        assert_has_duplicated_trailer(
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
    fn has_duplicated_trailer_one_trailer_is_fine() {
        assert_has_duplicated_trailer(
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
    fn has_duplicated_trailer_the_trailer_has_to_have_a_colon_to_count() {
        assert_has_duplicated_trailer(
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
    fn has_duplicated_trailer_the_trailer_can_be_anything() {
        assert_has_duplicated_trailer(
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

#[cfg(test)]
mod tests_has_missing_pivotal_tracker_id {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn has_missing_pivotal_tracker_id_with_id_is_fine() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678]
    "#,
            None,
        );
    }

    fn assert_has_missing_pivotal_tracker_id(message: &str, expected: Option<()>) {
        let actual = has_missing_pivotal_tracker_id(message);
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_id_with_multiple_ids_is_fine() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678,#87654321]
    "#,
            None,
        );
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678,#87654321,#11223344]
    "#,
            None,
        );
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678 #87654321 #11223344]
    "#,
            None,
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_id_with_a_state_fix_change_is_fine() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fix #12345678]
    "#,
            None,
        );
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[FIX #12345678]
    "#,
            None,
        );
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[Fix #12345678]
    "#,
            None,
        );
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fixed #12345678]
    "#,
            None,
        );
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fixes #12345678]
    "#,
            None,
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_id_with_a_complete_state_change_is_fine() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[complete #12345678]
    "#,
            None,
        );

        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[completed #12345678]
    "#,
            None,
        );

        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[Completed #12345678]
    "#,
            None,
        );

        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[completes #12345678]
    "#,
            None,
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_id_with_a_finish_state_change_is_fine() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finish #12345678]
    "#,
            None,
        );

        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finished #12345678]
    "#,
            None,
        );
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finishes #12345678]
    "#,
            None,
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_id_with_a_deliver_state_change_is_fine() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[deliver #12345678]
    "#,
            None,
        );

        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[delivered #12345678]
    "#,
            None,
        );
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[delivers #12345678]
    "#,
            None,
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_id_with_a_state_change_and_multiple_ids_is_fine() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fix #12345678 #12345678]
    "#,
            None,
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_id_with_prefixing_text_is_fine() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

Finally [fix #12345678 #12345678]
    "#,
            None,
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_with_a_fake_verb_does_not_work() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fake #12345678]
    "#,
            Some(()),
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_without_an_id_is_bad() {
        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit
    "#,
            Some(()),
        );

        assert_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#]
    "#,
            Some(()),
        );
    }
}

#[cfg(test)]
mod tests_get_lint_configuration {
    use pretty_assertions::assert_eq;

    use crate::{
        config::InMemoryVcs,
        lints::{
            get_lint_configuration,
            Lints,
            Lints::{DuplicatedTrailers, PivotalTrackerIdMissing},
        },
    };
    use std::collections::HashMap;

    #[test]
    fn with_no_config_return_a_hash_map_default_lints() {
        let git2_config = InMemoryVcs::new(HashMap::new(), HashMap::new(), HashMap::new());
        let actual =
            get_lint_configuration(&git2_config).expect("To be able to get a configuration");

        let expected = vec![DuplicatedTrailers];
        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn duplicate_trailer_detection_can_be_disabled() {
        let mut bool_configs = HashMap::new();
        bool_configs.insert("pb.lint.duplicated-trailers".into(), false);
        let git2_config = InMemoryVcs::new(bool_configs, HashMap::new(), HashMap::new());

        let actual =
            get_lint_configuration(&git2_config).expect("To be able to get a configuration");
        let expected: Vec<Lints> = vec![];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn duplicate_trailer_detection_can_be_explicitly_enabled() {
        let mut bool_configs = HashMap::new();
        bool_configs.insert("pb.lint.duplicated-trailers".into(), true);
        let git2_config = InMemoryVcs::new(bool_configs, HashMap::new(), HashMap::new());

        let actual =
            get_lint_configuration(&git2_config).expect("To be able to get a configuration");
        let expected: Vec<Lints> = vec![DuplicatedTrailers];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn pivotal_tracker_id_being_missing_can_be_explicitly_enabled() {
        let mut bool_configs = HashMap::new();
        bool_configs.insert("pb.lint.pivotal-tracker-id-missing".into(), true);
        let git2_config = InMemoryVcs::new(bool_configs, HashMap::new(), HashMap::new());
        let actual =
            get_lint_configuration(&git2_config).expect("To be able to get a configuration");
        let expected: Vec<Lints> = vec![DuplicatedTrailers, PivotalTrackerIdMissing];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }
}
