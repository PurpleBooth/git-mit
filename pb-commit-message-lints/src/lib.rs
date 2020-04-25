use std::collections::HashSet;

use std::iter::FromIterator;

pub fn has_duplicated_trailers(commit_message: &str, trailer: &str) -> bool {
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

    #[test]
    fn has_duplicated_trailers_no_trailer_is_fine() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers
"#;

        let actual = has_duplicated_trailers(commit_message, "Signed-off-by");
        assert_eq!(actual, false);
    }

    #[test]
    fn has_duplicated_trailers_two_identical_trailers_is_bad() {
        let commit_message = r#"
An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailers(commit_message, "Signed-off-by");
        assert_eq!(actual, true);
    }

    #[test]
    fn has_duplicated_trailers_two_trailers_with_different_names_is_fine() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <billie@example.com>
Signed-off-by: Ada Lovelace <ada@example.com>
"#;

        let actual = has_duplicated_trailers(commit_message, "Signed-off-by");
        assert_eq!(actual, false);
    }

    #[test]
    fn one_trailer_is_fine() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailers(commit_message, "Signed-off-by");
        assert_eq!(actual, false);
    }

    #[test]
    fn has_duplicated_trailers_the_trailer_has_to_have_a_colon_to_count() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers

Signed-off-by Billie Thompson <email@example.com>
Signed-off-by Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailers(commit_message, "Signed-off-by");
        assert_eq!(actual, false);
    }

    #[test]
    fn has_duplicated_trailers_the_trailer_can_be_anything() {
        let commit_message = r#"
An example commit

This is an example commit with duplicate trailers

Anything: Billie Thompson <email@example.com>
Anything: Billie Thompson <email@example.com>
"#;

        let actual = has_duplicated_trailers(commit_message, "Anything");
        assert_eq!(actual, true);
    }
}
