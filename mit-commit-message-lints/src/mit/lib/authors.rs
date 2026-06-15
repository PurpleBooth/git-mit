use std::{
    collections::{btree_map::IntoIter, BTreeMap, HashSet},
    convert::TryFrom,
};

use crate::mit::lib::{
    author::Author,
    errors::{DeserializeAuthorsError, SerializeAuthorsError},
};

/// Collection of authors
#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Authors<'a> {
    /// A btree of the authors
    pub authors: BTreeMap<String, Author<'a>>,
}

impl<'a> Authors<'a> {
    /// From a list of initials get the ones that aren't in our config
    #[must_use]
    pub fn missing_initials(&'a self, authors_initials: Vec<&'a str>) -> Vec<&'a str> {
        let configured: HashSet<_> = self.authors.keys().map(String::as_str).collect();
        let from_cli: HashSet<_> = authors_initials.into_iter().collect();
        from_cli.difference(&configured).copied().collect()
    }

    /// Create a new author collection
    #[must_use]
    pub const fn new(authors: BTreeMap<String, Author<'a>>) -> Self {
        Self { authors }
    }

    /// Get some authors by their initials
    #[must_use]
    pub fn get(&self, author_initials: &'a [&'a str]) -> Vec<&'a Author<'_>> {
        author_initials
            .iter()
            .filter_map(|initial| self.authors.get(*initial))
            .collect()
    }

    /// Merge two lists of authors
    ///
    /// This is used if the user has an author config file, and the authors are
    /// also saved in the vcs config
    #[must_use]
    pub fn merge(&self, authors: &Self) -> Self {
        let mut merged = self.authors.clone();
        merged.extend(authors.authors.clone());
        Self { authors: merged }
    }

    /// Generate an example authors list
    ///
    /// Used to show the user what their config file might look like
    #[must_use]
    pub fn example() -> Self {
        let mut store = BTreeMap::new();
        store.insert(
            "ae".into(),
            Author::new("Anyone Else".into(), "anyone@example.com".into(), None),
        );
        store.insert(
            "se".into(),
            Author::new("Someone Else".into(), "someone@example.com".into(), None),
        );
        store.insert(
            "bt".into(),
            Author::new(
                "Billie Thompson".into(),
                "billie@example.com".into(),
                Some("0A46826A".into()),
            ),
        );
        Self::new(store)
    }
}

impl<'a> IntoIterator for Authors<'a> {
    type Item = (String, Author<'a>);
    type IntoIter = IntoIter<String, Author<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.authors.into_iter()
    }
}

impl<'a> TryFrom<&'a str> for Authors<'a> {
    type Error = DeserializeAuthorsError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        serde_yaml::from_str(input)
            .or_else(|yaml_error| {
                toml::from_str(input).map_err(|toml_error| {
                    DeserializeAuthorsError::new(input, &yaml_error, &toml_error)
                })
            })
            .map(Self::new)
    }
}

impl TryFrom<String> for Authors<'_> {
    type Error = DeserializeAuthorsError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&input)
            .or_else(|yaml_error| {
                toml::from_str(&input).map_err(|toml_error| {
                    DeserializeAuthorsError::new(&input, &yaml_error, &toml_error)
                })
            })
            .map(Authors::new)
    }
}

impl<'a> TryFrom<Authors<'a>> for String {
    type Error = SerializeAuthorsError;

    fn try_from(value: Authors<'a>) -> Result<Self, Self::Error> {
        toml::to_string(&value.authors).map_err(SerializeAuthorsError)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use std::{
        collections::BTreeMap,
        convert::{TryFrom, TryInto},
    };

    use indoc::indoc;

    use crate::{
        external::InMemory,
        mit::{lib::author::Author, Authors},
    };

    #[test]
    fn is_is_iterable() {
        let mut store = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "billie@example.com".into(), None),
        );
        let actual = Authors::new(store);

        assert_eq!(
            actual.into_iter().collect::<Vec<_>>(),
            vec![(
                "bt".to_string(),
                Author::new("Billie Thompson".into(), "billie@example.com".into(), None)
            )],
            "Expected iterating to yield the single author with key 'bt'"
        );
    }

    #[test]
    fn it_can_get_an_author_in_it() {
        let mut store = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "billie@example.com".into(), None),
        );
        let actual = Authors::new(store);

        assert_eq!(
            actual.get(&["bt"]),
            vec![&Author::new(
                "Billie Thompson".into(),
                "billie@example.com".into(),
                None
            )],
            "Expected get by initials to return the matching author"
        );
    }

    #[test]
    fn i_can_get_multiple_authors_out_at_the_same_time() {
        let mut store: BTreeMap<String, Author<'_>> = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "billie@example.com".into(), None),
        );
        store.insert(
            "se".into(),
            Author::new("Somebody Else".into(), "somebody@example.com".into(), None),
        );
        let actual = Authors::new(store);

        assert_eq!(
            actual.get(&["bt"]),
            vec![&Author::new(
                "Billie Thompson".into(),
                "billie@example.com".into(),
                None
            )],
            "Expected get by 'bt' to return Billie Thompson"
        );
        assert_eq!(
            actual.get(&["se"]),
            vec![&Author::new(
                "Somebody Else".into(),
                "somebody@example.com".into(),
                None
            )],
            "Expected get by 'se' to return Somebody Else"
        );
    }

    #[test]
    fn there_is_an_example_constructor() {
        let mut store = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new(
                "Billie Thompson".into(),
                "billie@example.com".into(),
                Some("0A46826A".into()),
            ),
        );
        store.insert(
            "se".into(),
            Author::new("Someone Else".into(), "someone@example.com".into(), None),
        );
        store.insert(
            "ae".into(),
            Author::new("Anyone Else".into(), "anyone@example.com".into(), None),
        );
        let expected = Authors::new(store);

        assert_eq!(
            Authors::example(),
            expected,
            "Expected the example constructor to produce the predefined set of authors"
        );
    }

    #[test]
    fn merge_multiple_authors_together() {
        let mut map1: BTreeMap<String, Author<'_>> = BTreeMap::new();
        map1.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "billie@example.com".into(), None),
        );
        map1.insert(
            "se".into(),
            Author::new("Someone Else".into(), "someone@example.com".into(), None),
        );
        let input1: Authors<'_> = Authors::new(map1);

        let mut map2: BTreeMap<String, Author<'_>> = BTreeMap::new();
        map2.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "bt@example.com".into(), None),
        );
        map2.insert(
            "ae".into(),
            Author::new("Anyone Else".into(), "anyone@example.com".into(), None),
        );
        let input2: Authors<'_> = Authors::new(map2);

        let mut expected_map: BTreeMap<String, Author<'_>> = BTreeMap::new();

        expected_map.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "bt@example.com".into(), None),
        );
        expected_map.insert(
            "se".into(),
            Author::new("Someone Else".into(), "someone@example.com".into(), None),
        );
        expected_map.insert(
            "ae".into(),
            Author::new("Anyone Else".into(), "anyone@example.com".into(), None),
        );

        let expected: Authors<'_> = Authors::new(expected_map);

        assert_eq!(
            expected,
            input1.merge(&input2),
            "Expected the merged authors to contain entries from both inputs, with input2 taking precedence"
        );
    }

    #[test]
    fn it_can_tell_me_if_initials_are_not_in() {
        let mut store = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "billie@example.com".into(), None),
        );
        let actual = Authors::new(store);

        assert_eq!(
            actual.missing_initials(vec!["bt", "an"]),
            vec!["an"],
            "Expected only 'an' to be missing since 'bt' is configured"
        );
    }

    #[test]
    fn must_be_valid_yaml() {
        let actual: Result<_, _> = Authors::try_from("Hello I am invalid yaml : : :");
        actual.unwrap_err();
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

        let mut input: BTreeMap<String, Author<'_>> = BTreeMap::new();
        input.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "billie@example.com".into(), None),
        );
        let expected = Authors::new(input);

        assert_eq!(
            expected, actual,
            "Expected the parsed TOML to match the author for 'bt'"
        );
    }

    #[test]
    fn an_empty_file_is_a_default_authors() {
        let actual = Authors::try_from("").expect("Failed to parse yaml");

        let expected = Authors::default();

        assert_eq!(
            expected, actual,
            "Expected an empty file to parse as the default (empty) authors"
        );
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

        let mut input: BTreeMap<String, Author<'_>> = BTreeMap::new();
        input.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "billie@example.com".into(), None),
        );
        let expected = Authors::new(input);

        assert_eq!(
            expected, actual,
            "Expected the parsed YAML to match the author for 'bt'"
        );
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

        let mut expected_authors: BTreeMap<String, Author<'_>> = BTreeMap::new();
        expected_authors.insert(
            "bt".into(),
            Author::new(
                "Billie Thompson".into(),
                "billie@example.com".into(),
                Some("0A46826A".into()),
            ),
        );
        let expected = Authors::new(expected_authors);

        assert_eq!(
            expected, actual,
            "Expected the parsed YAML to include the signing key for 'bt'"
        );
    }

    #[test]
    fn it_converts_to_standard_toml() {
        let mut map: BTreeMap<String, Author<'_>> = BTreeMap::new();
        map.insert(
            "bt".into(),
            Author::new("Billie Thompson".into(), "billie@example.com".into(), None),
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

        assert_eq!(
            expected, actual,
            "Expected the serialized TOML to match the standard format without signing key"
        );
    }

    #[test]
    fn it_includes_the_signing_key_if_set() {
        let mut map: BTreeMap<String, Author<'_>> = BTreeMap::new();
        map.insert(
            "bt".into(),
            Author::new(
                "Billie Thompson".into(),
                "billie@example.com".into(),
                Some("0A46826A".into()),
            ),
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

        assert_eq!(
            expected, actual,
            "Expected the serialized TOML to include the signing key when set"
        );
    }

    #[test]
    fn it_can_give_me_an_author() {
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert("mit.author.config.zy.email".into(), "zy@example.com".into());
        strings.insert("mit.author.config.zy.name".into(), "Z Y".into());
        let vcs = InMemory::new(&mut strings);

        let actual = Authors::try_from(&vcs).expect("Failed to read VCS config");
        let expected_author = Author::new("Z Y".into(), "zy@example.com".into(), None);
        let mut store = BTreeMap::new();
        store.insert("zy".into(), expected_author);
        let expected = Authors::new(store);
        assert_eq!(
            expected, actual,
            "Expected the mit config to be {expected:?}, instead got {actual:?}"
        );
    }

    #[test]
    fn it_can_give_me_multiple_authors() {
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert("mit.author.config.zy.email".into(), "zy@example.com".into());
        strings.insert("mit.author.config.zy.name".into(), "Z Y".into());
        strings.insert(
            "mit.author.config.bt.email".into(),
            "billie@example.com".into(),
        );
        strings.insert("mit.author.config.bt.name".into(), "Billie Thompson".into());
        strings.insert("mit.author.config.bt.signingkey".into(), "ABC".into());
        let vcs = InMemory::new(&mut strings);

        let actual = Authors::try_from(&vcs).expect("Failed to read VCS config");
        let mut store = BTreeMap::new();
        store.insert(
            "zy".into(),
            Author::new("Z Y".into(), "zy@example.com".into(), None),
        );
        store.insert(
            "bt".into(),
            Author::new(
                "Billie Thompson".into(),
                "billie@example.com".into(),
                Some("ABC".into()),
            ),
        );
        let expected = Authors::new(store);
        assert_eq!(
            expected, actual,
            "Expected the mit config to be {expected:?}, instead got {actual:?}"
        );
    }

    #[test]
    fn broken_authors_are_skipped() {
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert("mit.author.config.zy.name".into(), "Z Y".into());
        strings.insert(
            "mit.author.config.bt.email".into(),
            "billie@example.com".into(),
        );
        strings.insert("mit.author.config.bt.name".into(), "Billie Thompson".into());
        strings.insert("mit.author.config.bt.signingkey".into(), "ABC".into());
        let vcs = InMemory::new(&mut strings);

        let actual = Authors::try_from(&vcs).expect("Failed to read VCS config");
        let mut store = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new(
                "Billie Thompson".into(),
                "billie@example.com".into(),
                Some("ABC".into()),
            ),
        );
        let expected = Authors::new(store);
        assert_eq!(
            expected, actual,
            "Expected the mit config to be {expected:?}, instead got {actual:?}"
        );
    }

    #[test]
    fn malformed_config_key_does_not_panic() {
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert("mit.author.config.".into(), "value".into());
        let vcs = InMemory::new(&mut strings);

        let result = Authors::try_from(&vcs);
        assert!(
            result.is_err(),
            "Expected an error for malformed config key"
        );
    }
}
