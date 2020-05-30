use std::error::Error;

use crate::author::entities::Authors;
use std::convert::TryFrom;

impl TryFrom<&str> for Authors {
    type Error = Box<dyn Error>;

    fn try_from(yaml: &str) -> Result<Self, Self::Error> {
        serde_yaml::from_str(yaml)
            .map_err(Box::from)
            .map(Authors::new)
    }
}

#[cfg(test)]
mod tests_able_to_load_config_from_yaml {
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;

    use crate::author::entities::{Author, Authors};
    use std::convert::TryFrom;

    #[test]
    fn must_be_valid_yaml() {
        let actual = Authors::try_from("Hello I am invalid yaml : : :");
        assert_eq!(true, actual.is_err())
    }

    #[test]
    fn it_can_parse_a_standard_yaml_file() {
        let actual = Authors::try_from(
            r#"---
bt:
    name: Billie Thompson
    email: billie@example.com
"#,
        );

        let mut input: HashMap<String, Author> = HashMap::new();
        input.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        let expected = Authors::new(input);

        assert_eq!(true, actual.is_ok());
        assert_eq!(expected, actual.unwrap());
    }

    #[test]
    fn yaml_files_can_contain_signing_keys() {
        let actual = Authors::try_from(
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
        assert_eq!(expected, actual.unwrap());
    }
}
