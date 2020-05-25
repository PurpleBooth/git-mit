use std::{collections::HashSet, error::Error, iter::FromIterator};

use enum_iterator::IntoEnumIterator;
use regex::Regex;

use crate::{
    external::vcs::Vcs,
    lints::Lints::{DuplicatedTrailers, PivotalTrackerIdMissing},
};

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 2] = ["Signed-off-by", "Co-authored-by"];
const REGEX_PIVOTAL_TRACKER_ID: &str =
    r"\[(((finish|fix)(ed|es)?|complete[ds]?|deliver(s|ed)?) )?#\d+([, ]#\d+)*]";

/// The lints that are supported
#[derive(Debug, Eq, PartialEq, IntoEnumIterator, Copy, Clone)]
pub enum Lints {
    DuplicatedTrailers,
    PivotalTrackerIdMissing,
}

const CONFIG_DUPLICATED_TRAILERS: &str = "duplicated-trailers";
const CONFIG_PIVOTAL_TRACKER_ID_MISSING: &str = "pivotal-tracker-id-missing";

impl std::fmt::Display for Lints {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", to_static_string(*self))
    }
}

impl std::convert::TryFrom<&str> for Lints {
    type Error = Box<dyn Error>;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Lints::into_enum_iter()
            .zip(Lints::into_enum_iter().map(|lint| lint.into()))
            .filter_map(|(lint, name): (Lints, &str)| if name == from { Some(lint) } else { None })
            .collect::<Vec<Lints>>()
            .first()
            .copied()
            .ok_or_else(|| -> Box<dyn Error> {
                format!("Could not match {} to a lint", from).into()
            })
    }
}

impl std::convert::From<Lints> for &'static str {
    fn from(from: Lints) -> Self {
        to_static_string(from)
    }
}

impl std::convert::From<Lints> for String {
    fn from(from: Lints) -> Self {
        String::from(to_static_string(from))
    }
}

#[cfg(test)]
mod tests_lints {
    use std::convert::TryInto;

    use pretty_assertions::assert_eq;

    use crate::lints::{Lints, Lints::PivotalTrackerIdMissing};

    #[test]
    fn it_is_convertible_to_string() {
        let string: String = Lints::PivotalTrackerIdMissing.into();
        assert_eq!("pivotal-tracker-id-missing".to_string(), string)
    }

    #[test]
    fn it_can_be_created_from_string() {
        let lint: Lints = "pivotal-tracker-id-missing".try_into().unwrap();
        assert_eq!(PivotalTrackerIdMissing, lint)
    }

    #[test]
    fn it_is_printable() {
        assert_eq!(
            "pivotal-tracker-id-missing",
            &format!("{}", Lints::PivotalTrackerIdMissing)
        )
    }
}

/// Look at a git config and work out what lints should be turned on and off
///
/// # Example
///
/// ```
/// use git2::Repository;
/// use pb_commit_message_lints::{
///     external::vcs::Git2,
///     lints::{get_lint_configuration, Lints::DuplicatedTrailers},
/// };
/// use tempfile::TempDir;
/// let config = TempDir::new()
///     .map(TempDir::into_path)
///     .map(|path| path.join("repository"))
///     .map(Repository::init)
///     .expect("Failed to initialise the repository")
///     .expect("Failed create temporary directory")
///     .config()
///     .map(Git2::new)
///     .expect("Failed to get configuration");
///
/// assert_eq!(get_lint_configuration(&config), vec![DuplicatedTrailers],)
/// ```
///
/// # Errors
///
/// Will return `Err` if we can't read the git configuration for some reason or it's not parsable
pub fn get_lint_configuration(config: &dyn Vcs) -> Vec<Lints> {
    vec![
        config
            .get_bool(&format!("pb.lint.{}", Lints::DuplicatedTrailers))
            .or(Some(true))
            .filter(bool::clone)
            .map(|_| DuplicatedTrailers),
        config
            .get_bool(&format!("pb.lint.{}", Lints::PivotalTrackerIdMissing))
            .filter(bool::clone)
            .map(|_| PivotalTrackerIdMissing),
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// Check if a commit message message has duplicated trailers with names in
///
/// # Example
///
/// ```
/// use pb_commit_message_lints::lints::has_duplicated_trailers;
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
    Some(
        TRAILERS_TO_CHECK_FOR_DUPLICATES
            .iter()
            .filter_map(|trailer| filter_without_duplicates(commit_message, trailer))
            .collect::<Vec<String>>(),
    )
    .filter(|duplicates: &Vec<_>| !duplicates.is_empty())
}

fn filter_without_duplicates(commit_message: &str, trailer: &str) -> Option<String> {
    Some(trailer)
        .map(String::from)
        .filter(|trailer| has_duplicated_trailer(commit_message, trailer))
}

/// Check if a commit message message has a pivotal tracker id in it
///
/// # Example
///
/// ```
/// use pb_commit_message_lints::lints::has_missing_pivotal_tracker_id;
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
    Some(commit_message)
        .map(str::to_lowercase)
        .filter(|message| has_no_pivotal_tracker_id(message))
        .map(|_| ())
}

fn has_no_pivotal_tracker_id(text: &str) -> bool {
    Regex::new(REGEX_PIVOTAL_TRACKER_ID)
        .map(|re| !re.is_match(&text))
        .unwrap()
}

fn has_duplicated_trailer(commit_message: &str, trailer: &str) -> bool {
    Some(
        commit_message
            .lines()
            .filter(|line: &&str| has_trailer(trailer, line))
            .collect::<Vec<_>>(),
    )
    .map(|trailers| (trailers.clone(), trailers.clone()))
    .map(|(commit, unique)| (commit, HashSet::<&str>::from_iter(unique)))
    .map(|(commit, unique)| commit.len() != unique.len())
    .unwrap()
}

fn has_trailer(trailer: &str, line: &&str) -> bool {
    line.starts_with(&format!("{}:", trailer))
}

#[cfg(test)]
mod tests_has_duplicated_trailers {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn has_duplicated_trailers_runs_both_co_authored_and_signed_off_by() {
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers
"#,
            &None,
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
            &Some(vec!["Signed-off-by".into(), "Co-authored-by".into()]),
        );
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#,
            &Some(vec!["Signed-off-by".into()]),
        );
        test_has_duplicated_trailers(
            r#"
An example commit

This is an example commit without any duplicate trailers

Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#,
            &Some(vec!["Co-authored-by".into()]),
        );
    }

    fn test_has_duplicated_trailers(message: &str, expected: &Option<Vec<String>>) {
        let actual = has_duplicated_trailers(message);
        assert_eq!(
            actual, *expected,
            "Expected {:?}, found {:?}",
            expected, actual
        );
    }

    fn test_has_duplicated_trailer(message: &str, trailer: &str, expected: bool) {
        let actual = has_duplicated_trailer(message, trailer);
        assert_eq!(
            actual, expected,
            "Message {:?} with trailer {:?} should have returned {:?}, found {:?}",
            message, trailer, expected, actual
        );
    }

    #[test]
    fn has_duplicated_trailer_does_nothing_on_no_trailer() {
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
    fn has_duplicated_trailer_detects_duplicated_trailers() {
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
    fn has_duplicated_trailer_two_trailers_with_different_names_is_fine() {
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
    fn has_duplicated_trailer_one_trailer_is_fine() {
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
    fn has_duplicated_trailer_the_trailer_has_to_have_a_colon_to_count() {
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
    fn has_duplicated_trailer_the_trailer_can_be_anything() {
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

#[cfg(test)]
mod tests_has_missing_pivotal_tracker_id {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn has_missing_pivotal_tracker_id_with_id_is_fine() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678]
    "#,
            None,
        );
    }

    fn test_has_missing_pivotal_tracker_id(message: &str, expected: Option<()>) {
        let actual = has_missing_pivotal_tracker_id(message);
        assert_eq!(
            actual, expected,
            "Message {:?} should have returned {:?}, found {:?}",
            message, expected, actual
        );
    }

    #[test]
    fn has_missing_pivotal_tracker_id_with_multiple_ids_is_fine() {
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678,#87654321]
    "#,
            None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[#12345678,#87654321,#11223344]
    "#,
            None,
        );
        test_has_missing_pivotal_tracker_id(
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
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fix #12345678]
    "#,
            None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[FIX #12345678]
    "#,
            None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[Fix #12345678]
    "#,
            None,
        );
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[fixed #12345678]
    "#,
            None,
        );
        test_has_missing_pivotal_tracker_id(
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
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[complete #12345678]
    "#,
            None,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[completed #12345678]
    "#,
            None,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[Completed #12345678]
    "#,
            None,
        );

        test_has_missing_pivotal_tracker_id(
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
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finish #12345678]
    "#,
            None,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[finished #12345678]
    "#,
            None,
        );
        test_has_missing_pivotal_tracker_id(
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
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[deliver #12345678]
    "#,
            None,
        );

        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit

[delivered #12345678]
    "#,
            None,
        );
        test_has_missing_pivotal_tracker_id(
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
        test_has_missing_pivotal_tracker_id(
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
        test_has_missing_pivotal_tracker_id(
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
        test_has_missing_pivotal_tracker_id(
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
        test_has_missing_pivotal_tracker_id(
            r#"
An example commit

This is an example commit
    "#,
            Some(()),
        );

        test_has_missing_pivotal_tracker_id(
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
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use crate::{
        external::vcs::InMemory,
        lints::{
            get_lint_configuration,
            Lints,
            Lints::{DuplicatedTrailers, PivotalTrackerIdMissing},
        },
    };

    #[test]
    fn with_no_config_return_a_hash_map_default_lints() {
        let mut bools = HashMap::new();
        let mut strings = HashMap::new();
        let mut i64s = HashMap::new();
        let config = InMemory::new(&mut bools, &mut strings, &mut i64s);

        let actual = get_lint_configuration(&config);
        let expected = vec![DuplicatedTrailers];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn duplicate_trailer_detection_can_be_disabled() {
        let mut bools = HashMap::new();
        bools.insert("pb.lint.duplicated-trailers".into(), false);
        let mut strings = HashMap::new();
        let mut i64s = HashMap::new();
        let config = InMemory::new(&mut bools, &mut strings, &mut i64s);

        let actual = get_lint_configuration(&config);
        let expected: Vec<Lints> = vec![];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn duplicate_trailer_detection_can_be_explicitly_enabled() {
        let mut bools = HashMap::new();
        bools.insert("pb.lint.duplicated-trailers".into(), true);
        let mut strings = HashMap::new();
        let mut i64s = HashMap::new();
        let config = InMemory::new(&mut bools, &mut strings, &mut i64s);

        let actual = get_lint_configuration(&config);
        let expected: Vec<Lints> = vec![DuplicatedTrailers];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn pivotal_tracker_id_being_missing_can_be_explicitly_enabled() {
        let mut bools = HashMap::new();
        bools.insert("pb.lint.pivotal-tracker-id-missing".into(), true);
        let mut strings = HashMap::new();
        let mut i64s = HashMap::new();
        let config = InMemory::new(&mut bools, &mut strings, &mut i64s);

        let actual = get_lint_configuration(&config);
        let expected: Vec<Lints> = vec![DuplicatedTrailers, PivotalTrackerIdMissing];

        assert_eq!(
            expected, actual,
            "Expected the list of lint identifiers to be {:?}, instead got {:?}",
            expected, actual
        )
    }
}

fn to_static_string(lint: Lints) -> &'static str {
    match lint {
        Lints::DuplicatedTrailers => CONFIG_DUPLICATED_TRAILERS,
        Lints::PivotalTrackerIdMissing => CONFIG_PIVOTAL_TRACKER_ID_MISSING,
    }
}

#[cfg(test)]
mod tests_can_enable_lints_via_a_command {
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use crate::{
        external::vcs::InMemory,
        lints::{set_lint_status, Lints::PivotalTrackerIdMissing},
    };

    #[test]
    fn we_can_enable_lints() {
        let mut bools = HashMap::new();
        bools.insert("pb.lint.pivotal-tracker-id-missing".into(), false);
        let mut strings = HashMap::new();
        let mut i64s = HashMap::new();
        let mut config = InMemory::new(&mut bools, &mut strings, &mut i64s);

        set_lint_status(&[PivotalTrackerIdMissing], &mut config, true).unwrap();

        let expected = "true".to_string();
        let actual = strings
            .get("pb.lint.pivotal-tracker-id-missing")
            .unwrap()
            .clone();
        assert_eq!(expected, actual);
    }

    #[test]
    fn we_can_disable_lints() {
        let mut bools = HashMap::new();
        bools.insert("pb.lint.pivotal-tracker-id-missing".into(), true);
        let mut strings = HashMap::new();
        let mut i64s = HashMap::new();
        let mut config = InMemory::new(&mut bools, &mut strings, &mut i64s);

        set_lint_status(&[PivotalTrackerIdMissing], &mut config, false).unwrap();

        let expected = "false".to_string();
        let actual = strings
            .get("pb.lint.pivotal-tracker-id-missing")
            .unwrap()
            .clone();
        assert_eq!(expected, actual);
    }
}

/// # Errors
///
/// Errors if writing to the VCS config fails
pub fn set_lint_status(
    lints: &[Lints],
    vcs: &mut dyn Vcs,
    status: bool,
) -> Result<(), Box<dyn Error>> {
    lints
        .iter()
        .try_for_each(|lint| vcs.set_str(&format!("pb.lint.{}", lint), &status.to_string()))
}
