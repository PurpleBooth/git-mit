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
    /// From a list of initials get te ones that aren't in our config
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
    /// This is used if the user has a author config file, and the authors are
    /// also saved in the vcs config
    #[must_use]
    pub fn merge(&self, authors: &Self) -> Self {
        Self {
            authors: authors
                .authors
                .iter()
                .fold(self.authors.clone(), |mut acc, (key, value)| {
                    acc.insert(key.clone(), value.clone());
                    acc
                }),
        }
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
    type IntoIter = IntoIter<String, Author<'a>>;
    type Item = (String, Author<'a>);

    fn into_iter(self) -> Self::IntoIter {
        self.authors.into_iter()
    }
}

impl<'a> TryFrom<&'a str> for Authors<'a> {
    type Error = DeserializeAuthorsError;

    fn try_from(input: &str) -> std::result::Result<Self, Self::Error> {
        serde_yaml::from_str(input)
            .or_else(|yaml_error| {
                toml::from_str(input).map_err(|toml_error| {
                    DeserializeAuthorsError::new(input, &yaml_error, &toml_error)
                })
            })
            .map(Self::new)
    }
}

impl<'a> TryFrom<String> for Authors<'a> {
    type Error = DeserializeAuthorsError;

    fn try_from(input: String) -> std::result::Result<Self, Self::Error> {
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
