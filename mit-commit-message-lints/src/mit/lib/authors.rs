use std::collections::BTreeMap;

use crate::mit::lib::author::Author;
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Authors {
    pub authors: BTreeMap<String, Author>,
}

impl Authors {
    #[must_use]
    pub fn new(authors: BTreeMap<String, Author>) -> Authors {
        Authors { authors }
    }

    #[must_use]
    pub fn get(&self, author_initials: &[&str]) -> Vec<Option<&Author>> {
        author_initials
            .iter()
            .map(|initial| self.authors.get(*initial))
            .collect()
    }

    #[must_use]
    pub fn merge(&self, authors: &Authors) -> Authors {
        Authors {
            authors: authors
                .authors
                .iter()
                .fold(self.authors.clone(), |mut acc, (key, value)| {
                    acc.insert(key.clone(), value.clone());
                    acc
                }),
        }
    }

    #[must_use]
    pub fn example() -> Authors {
        let mut store = BTreeMap::new();
        store.insert(
            "ae".into(),
            Author::new("Anyone Else", "anyone@example.com", None),
        );
        store.insert(
            "se".into(),
            Author::new("Someone Else", "someone@example.com", None),
        );
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", Some("0A46826A")),
        );
        Authors::new(store)
    }
}

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

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to parse authors as toml {0} or as yaml {1}")]
    Parse(serde_yaml::Error, toml::de::Error),
    #[error("failed to serialise toml {0}")]
    SerialiseYaml(#[from] toml::ser::Error),
}

impl TryFrom<Authors> for String {
    type Error = Error;

    fn try_from(value: Authors) -> Result<Self, Self::Error> {
        toml::to_string(&value.authors).map_err(Error::from)
    }
}

#[cfg(test)]
mod tests_authors {
    #![allow(clippy::wildcard_imports)]

    use super::*;
    use crate::mit::lib::author::Author;
    use indoc::indoc;
    use std::convert::TryInto;

    #[test]
    fn it_can_get_an_author_in_it() {
        let mut store = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        let actual = Authors::new(store);

        assert_eq!(
            actual.get(&["bt"]),
            vec![Some(&Author::new(
                "Billie Thompson",
                "billie@example.com",
                None,
            ))]
        )
    }

    #[test]
    fn i_can_get_multiple_authors_out_at_the_same_time() {
        let mut store: BTreeMap<String, Author> = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        store.insert(
            "se".into(),
            Author::new("Somebody Else", "somebody@example.com", None),
        );
        let actual = Authors::new(store);

        assert_eq!(
            actual.get(&["bt"]),
            vec![Some(&Author::new(
                "Billie Thompson",
                "billie@example.com",
                None,
            ))]
        );
        assert_eq!(
            actual.get(&["se"]),
            vec![Some(&Author::new(
                "Somebody Else",
                "somebody@example.com",
                None,
            ))]
        )
    }

    #[test]
    fn there_is_an_example_constructor() {
        let mut store = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", Some("0A46826A")),
        );
        store.insert(
            "se".into(),
            Author::new("Someone Else", "someone@example.com", None),
        );
        store.insert(
            "ae".into(),
            Author::new("Anyone Else", "anyone@example.com", None),
        );
        let expected = Authors::new(store);

        assert_eq!(Authors::example(), expected,)
    }

    #[test]
    fn merge_multiple_authors_together() {
        let mut map1: BTreeMap<String, Author> = BTreeMap::new();
        map1.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", None),
        );
        map1.insert(
            "se".into(),
            Author::new("Someone Else", "someone@example.com", None),
        );
        let input1: Authors = Authors::new(map1);

        let mut map2: BTreeMap<String, Author> = BTreeMap::new();
        map2.insert(
            "bt".into(),
            Author::new("Billie Thompson", "bt@example.com", None),
        );
        map2.insert(
            "ae".into(),
            Author::new("Anyone Else", "anyone@example.com", None),
        );
        let input2: Authors = Authors::new(map2);

        let mut expected_map: BTreeMap<String, Author> = BTreeMap::new();

        expected_map.insert(
            "bt".into(),
            Author::new("Billie Thompson", "bt@example.com", None),
        );
        expected_map.insert(
            "se".into(),
            Author::new("Someone Else", "someone@example.com", None),
        );
        expected_map.insert(
            "ae".into(),
            Author::new("Anyone Else", "anyone@example.com", None),
        );

        let expected: Authors = Authors::new(expected_map);

        assert_eq!(expected, input1.merge(&input2));
    }

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
