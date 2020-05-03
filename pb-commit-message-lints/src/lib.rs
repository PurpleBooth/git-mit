use std::{collections::HashSet, error, iter::FromIterator};

use git2::Config;
use regex::Regex;

use crate::Lints::{DuplicatedTrailers, PivotalTrackerIdMissing};

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 2] = ["Signed-off-by", "Co-authored-by"];
const CONFIG_DUPLICATED_TRAILERS: &str = "pb.message.duplicated-trailers";
const CONFIG_PIVOTAL_TRACKER_ID_MISSING: &str = "pb.message.pivotal-tracker-id-missing";
const REGEX_PIVOTAL_TRACKER_ID: &str =
    r"\[(((finish|fix)(ed|es)?|complete[ds]?|deliver(s|ed)?) )?#\d+([, ]#\d+)*]";

/// The lints that are supported
#[derive(Debug, Eq, PartialEq)]
pub enum Lints {
    DuplicatedTrailers,
    PivotalTrackerIdMissing,
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Look at a git config and work out what lints should be turned on and off
///
/// # Example
///
/// ```
/// use git2::Repository;
/// use pb_commit_message_lints::{get_lint_configuration, Lints::DuplicatedTrailers};
/// use pretty_assertions::assert_eq;
/// use tempfile::TempDir;
///
/// let config = TempDir::new()
///     .map(TempDir::into_path)
///     .map(|x| x.join("repository"))
///     .map(Repository::init)
///     .expect("Failed to initialise the repository")
///     .expect("Failed create temporary directory")
///     .config()
///     .expect("Failed to get configuration");
///
/// let actual = get_lint_configuration(&config).expect("To be able to get a configuration");
///
/// let expected = vec![DuplicatedTrailers];
/// assert_eq!(
///     expected, actual,
///     "Expected the list of lint identifiers to be {:?}, instead got {:?}",
///     expected, actual
/// )
/// ```
///
/// # Errors
///
/// Will return `Err` if we can't read the git configuration for some reason or it's not parsable
pub fn get_lint_configuration(config: &Config) -> Result<Vec<Lints>> {
    let mut result_vec: Vec<Lints> = vec![];

    if !config_defined(config, CONFIG_DUPLICATED_TRAILERS)?
        || config.get_bool(CONFIG_DUPLICATED_TRAILERS)?
    {
        result_vec.push(DuplicatedTrailers)
    }

    if config_defined(config, CONFIG_PIVOTAL_TRACKER_ID_MISSING)?
        && config.get_bool(CONFIG_PIVOTAL_TRACKER_ID_MISSING)?
    {
        result_vec.push(PivotalTrackerIdMissing)
    }

    Ok(result_vec)
}

fn config_defined(config: &Config, lint_name: &str) -> Result<bool> {
    config
        .entries(Some(lint_name))
        .map(|x| x.count() > 0)
        .map_err(Box::from)
}

/// Check if a commit message message has duplicated trailers with names in
///
/// # Example
///
/// ```
/// use pb_commit_message_lints::has_duplicated_trailers;
///
/// let commit_message_with_repeating_signed_off_by = r#"
/// An example commit
///
/// This is an example commit without any duplicate trailers
///
/// Signed-off-by: Billie Thompson <email@example.com>
/// Signed-off-by: Billie Thompson <email@example.com>
/// "#;
/// let actual = has_duplicated_trailers(commit_message_with_repeating_signed_off_by);
/// assert_eq!(actual, Some(vec!["Signed-off-by".to_string()]));
///
/// let commit_message_with_repeating_co_authors = r#"
/// An example commit
///
/// This is an example commit without any duplicate trailers
///
/// Co-authored-by: Billie Thompson <email@example.com>
/// Co-authored-by: Billie Thompson <email@example.com>
/// "#;
///
/// let actual = has_duplicated_trailers(commit_message_with_repeating_co_authors);
/// assert_eq!(actual, Some(vec!["Co-authored-by".to_string()]));
/// ```
#[must_use]
pub fn has_duplicated_trailers(commit_message: &str) -> Option<Vec<String>> {
    let duplicated_trailers: Vec<String> = TRAILERS_TO_CHECK_FOR_DUPLICATES
        .iter()
        .filter_map(|x| {
            if !has_duplicated_trailer(commit_message, x) {
                return None;
            }

            Some((*x).to_string())
        })
        .collect();

    if !duplicated_trailers.is_empty() {
        return Some(duplicated_trailers);
    }

    None
}

/// Check if a commit message message has a pivotal tracker id in it
///
/// # Example
///
/// ```
/// use pb_commit_message_lints::has_missing_pivotal_tracker_id;
///
/// let message = r#"
/// [fix #12345678] correct bug the build
/// "#;
/// let actual = has_missing_pivotal_tracker_id(message);
/// assert_eq!(actual, None);
///
/// let message = r#"
/// Add a new commit linter
///
/// This will add a new linter. This linter checks for the presence of a Pivotal Tracker Id. In this
/// example I have forgotten my Id.
/// "#;
/// let actual = has_missing_pivotal_tracker_id(message);
/// assert_eq!(actual, Some(()));
///
/// let message = r#"
/// Add a new commit linter
///
/// This will add a new linter. This linter checks for the presence of a Pivotal Tracker Id. In this
/// example I have not forgotten my Id
///
/// [deliver #12345678,#88335556,#87654321]
/// "#;
/// let actual = has_missing_pivotal_tracker_id(message);
/// assert_eq!(actual, None);
/// ```
pub fn has_missing_pivotal_tracker_id(commit_message: &str) -> Option<()> {
    let re = Regex::new(REGEX_PIVOTAL_TRACKER_ID).unwrap();

    if !re.is_match(&commit_message.to_lowercase()) {
        return Some(());
    }

    None
}

fn has_duplicated_trailer(commit_message: &str, trailer: &str) -> bool {
    let trailers: Vec<&str> = commit_message
        .lines()
        .filter(|x| x.starts_with(&format!("{}:", trailer)))
        .collect();

    let unique_trailers: std::collections::HashSet<&str> =
        HashSet::from_iter(trailers.to_owned().into_iter());

    trailers.len() != unique_trailers.len()
}

#[cfg(test)]
mod tests {
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
            None,
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
            Some(vec![
                "Signed-off-by".to_string(),
                "Co-authored-by".to_string(),
            ]),
        );
        assert_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#,
            Some(vec!["Signed-off-by".to_string()]),
        );
        assert_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#,
            Some(vec!["Co-authored-by".to_string()]),
        );
    }

    fn assert_has_duplicated_trailers(message: &str, expected: Option<Vec<String>>) {
        let actual = has_duplicated_trailers(message);
        assert_eq!(
            actual, expected,
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
