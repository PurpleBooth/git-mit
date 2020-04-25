use std::collections::HashSet;

use crate::Lints::DuplicatedTrailers;
use git2::Config;
use std::error;
use std::iter::FromIterator;

const TRAILERS_TO_CHECK_FOR_DUPLICATES: [&str; 2] = ["Signed-off-by", "Co-authored-by"];

#[derive(Debug, Eq, PartialEq)]
pub enum Lints {
    DuplicatedTrailers,
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn get_lint_configuration(config: &Config) -> Result<Vec<Lints>> {
    let mut result_vec: Vec<Lints> = vec![];

    let defined = config
        .entries(Some("pb.message.duplicated-trailers"))
        .map(|x| x.count() > 0);

    match defined {
        Err(e) => return Err(Box::from(e)),
        Ok(false) => result_vec.push(DuplicatedTrailers),
        _ => {}
    }

    if let Ok(true) = config.get_bool("pb.message.duplicated-trailers") {
        result_vec.push(DuplicatedTrailers)
    }

    Ok(result_vec)
}

pub fn has_duplicated_trailers(commit_message: &str) -> Option<Vec<String>> {
    let duplicated_trailers: Vec<String> = TRAILERS_TO_CHECK_FOR_DUPLICATES
        .iter()
        .filter(|x| has_duplicated_trailer(commit_message, x))
        .map(|x| (*x).to_string())
        .collect();

    if !duplicated_trailers.is_empty() {
        return Some(duplicated_trailers);
    }

    None
}

pub fn has_duplicated_trailer(commit_message: &str, trailer: &str) -> bool {
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
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn has_duplicated_trailers_runs_both_tests() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers
"#;

        let actual = has_duplicated_trailers(commit_message);
        let expected = None;
        assert_eq!(
            actual, expected,
            "Expected {:?}, found {:?}",
            expected, actual
        );

        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailers(commit_message);
        let expected = Some(vec![
            "Signed-off-by".to_string(),
            "Co-authored-by".to_string(),
        ]);
        assert_eq!(actual, expected);

        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailers(commit_message);
        assert_eq!(actual, Some(vec!["Signed-off-by".to_string()]));

        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers

Co-authored-by: Billie Thompson <email@example.com>
Co-authored-by: Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailers(commit_message);
        assert_eq!(actual, Some(vec!["Co-authored-by".to_string()]));
    }

    #[test]
    fn has_duplicated_trailer_no_trailer_is_fine() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers
"#;

        let actual = has_duplicated_trailer(commit_message, "Signed-off-by");
        assert_eq!(actual, false);
    }

    #[test]
    fn has_duplicated_trailer_two_identical_trailers_is_bad() {
        let commit_message = r#"
An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailer(commit_message, "Signed-off-by");
        assert_eq!(actual, true);
    }

    #[test]
    fn has_duplicated_trailer_two_trailers_with_different_names_is_fine() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <billie@example.com>
Signed-off-by: Ada Lovelace <ada@example.com>
"#;

        let actual = has_duplicated_trailer(commit_message, "Signed-off-by");
        assert_eq!(actual, false);
    }

    #[test]
    fn one_trailer_is_fine() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailer(commit_message, "Signed-off-by");
        assert_eq!(actual, false);
    }

    #[test]
    fn has_duplicated_trailer_the_trailer_has_to_have_a_colon_to_count() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by Billie Thompson <email@example.com>
Signed-off-by Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailer(commit_message, "Signed-off-by");
        assert_eq!(actual, false);
    }

    #[test]
    fn has_duplicated_trailer_the_trailer_can_be_anything() {
        let commit_message = r#"
An example commit

This is an example commit with duplicate trailers

Anything: Billie Thompson <email@example.com>
Anything: Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailer(commit_message, "Anything");
        assert_eq!(actual, true);
    }
}
