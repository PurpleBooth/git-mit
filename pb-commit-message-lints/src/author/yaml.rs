use std::error::Error;

use crate::author::entities::Authors;

/// # Errors
///
/// Errors if the YAML isn't valid, or isn't valid for a Author map.
pub fn get_authors_from_user_config(yaml: &str) -> Result<Authors, Box<dyn Error>> {
    serde_yaml::from_str(yaml)
        .map_err(Box::from)
        .map(Authors::new)
}

#[cfg(test)]
mod tests_able_to_load_config_from_yaml {
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use crate::author::{
        entities::{Author, Authors},
        yaml::get_authors_from_user_config,
    };

    #[test]
    fn must_be_valid_yaml() {
        let actual = get_authors_from_user_config("Hello I am invalid yaml : : :");
        assert_eq!(true, actual.is_err())
    }

    #[test]
    fn it_can_parse_a_standard_yaml_file() {
        let actual = get_authors_from_user_config(
            r#"---
bt:
    name: Billie Thompson
    email: billie@example.com
"#,
        );

        let mut expected_authors: HashMap<String, Author> = HashMap::new();
        expected_authors.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        let expected = Authors::new(expected_authors);

        assert_eq!(true, actual.is_ok());
        let actual_authors = actual.unwrap();
        assert_eq!(expected, actual_authors);
    }

    #[test]
    fn yaml_files_can_contain_signing_keys() {
        let actual = get_authors_from_user_config(
            r#"---
bt:
    name: Billie Thompson
    email: billie@example.com
    signingkey: 0A46826A
"#,
        );

        let mut expected_authors: HashMap<String, Author> = HashMap::new();
        expected_authors.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", Some("0A46826A")),
        );
        let expected = Authors::new(expected_authors);

        assert_eq!(true, actual.is_ok());
        let actual_authors = actual.unwrap();
        assert_eq!(expected, actual_authors);
    }
}
