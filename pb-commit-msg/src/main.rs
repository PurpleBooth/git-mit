use clap::{crate_authors, crate_version, App, Arg};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::iter::FromIterator;

fn main() -> std::io::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("commit-file-path")
                .help("Path to a temporary file that contains the commit message written by the developer")
                .index(1)
                .required(true)
        )
        .get_matches();

    let commit_file_path = matches.value_of("commit-file-path").unwrap();
    let commit_message =
        fs::read_to_string(commit_file_path).expect("Something went wrong reading the file");

    for trailer in &["Signed-off-by", "Co-authored-by"] {
        if has_duplicated_trailers(&commit_message, trailer) {
            eprintln!(
                r#"
{}

Your commit cannot have the same name duplicated in the \"{}\" field
"#,
                commit_message, trailer
            );

            std::process::exit(1);
        }
    }

    Ok(())
}

fn has_duplicated_trailers(commit_message: &str, trailer: &str) -> bool {
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
    fn no_trailer_is_fine() {
        let commit_message = r#"
An example commit

This is an example commit without any duplicate trailers
"#;

        let actual = has_duplicated_trailers(commit_message, "Signed-off-by");
        assert_eq!(actual, false);
    }

    #[test]
    fn two_identical_trailers_is_bad() {
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
    fn two_trailers_with_different_names_is_fine() {
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
    fn the_trailer_has_to_have_a_colon_to_count() {
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
    fn the_trailer_can_be_anything() {
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
