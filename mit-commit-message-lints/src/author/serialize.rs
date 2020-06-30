use std::convert::TryFrom;

use thiserror::Error;

use crate::author::entities::Authors;

impl TryFrom<&str> for Authors {
    type Error = Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        serde_yaml::from_str(input)
            .or_else(|yaml_error| {
                toml::from_str(input).map_err(|toml_error| Error::Parse(yaml_error, toml_error))
            })
            .map_err(Error::from)
            .map(Authors::new)
    }
}

impl TryFrom<Authors> for String {
    type Error = Error;

    fn try_from(value: Authors) -> Result<Self, Self::Error> {
        toml::to_string(&value.authors).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::convert::TryFrom;
    use std::convert::TryInto;

    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use crate::author::entities::{Author, Authors};

    #[test]
    fn must_be_valid_yaml() {
        let actual: Result<_, _> = Authors::try_from("Hello I am invalid yaml : : :");
        assert_eq!(true, actual.is_err())
    }

    #[test]
    fn it_can_parse_a_standard_toml_file() {
        let actual = Authors::try_from(indoc!(
            "
            [bt]
            name = \"Billie Thompson\"
            email = \"billie@example.com\"
            "
        ))
        .expect("Failed to parse yaml");

        let mut input: BTreeMap<String, Author> = BTreeMap::new();
        input.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        let expected = Authors::new(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_can_parse_a_standard_yaml_file() {
        let actual = Authors::try_from(indoc!(
            "
            ---
            bt:
                name: Billie Thompson
                email: billie@example.com
            "
        ))
        .expect("Failed to parse yaml");

        let mut input: BTreeMap<String, Author> = BTreeMap::new();
        input.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        let expected = Authors::new(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn yaml_files_can_contain_signing_keys() {
        let actual = Authors::try_from(indoc!(
            "
            ---
            bt:
                name: Billie Thompson
                email: billie@example.com
                signingkey: 0A46826A
            "
        ))
        .expect("Failed to parse yaml");

        let mut expected_authors: BTreeMap<String, Author> = BTreeMap::new();
        expected_authors.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", Some("0A46826A")),
        );
        let expected = Authors::new(expected_authors);

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_converts_to_standard_toml() {
        let mut map: BTreeMap<String, Author> = BTreeMap::new();
        map.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        let actual: String = Authors::new(map).try_into().unwrap();
        let expected = indoc!(
            "
            [bt]
            name = \"Billie Thompson\"
            email = \"billie@example.com\"
            "
        )
        .to_string();

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_includes_the_signing_key_if_set() {
        let mut map: BTreeMap<String, Author> = BTreeMap::new();
        map.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", Some("0A46826A")),
        );
        let actual: String = Authors::new(map).try_into().unwrap();
        let expected = indoc!(
            "
            [bt]
            name = \"Billie Thompson\"
            email = \"billie@example.com\"
            signingkey = \"0A46826A\"
            "
        )
        .to_string();

        assert_eq!(expected, actual);
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to parse authors as toml {0} or as yaml {1}")]
    Parse(serde_yaml::Error, toml::de::Error),
    #[error("failed to serialise toml {0}")]
    SerialiseYaml(#[from] toml::ser::Error),
}
