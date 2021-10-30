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
pub struct Authors {
    /// A btree of the authors
    pub authors: BTreeMap<String, Author>,
}

impl Authors {
    /// From a list of initials get te ones that aren't in our config
    #[must_use]
    pub fn missing_initials<'a>(&'a self, authors_initials: Vec<&'a str>) -> Vec<&'a str> {
        let configured: HashSet<_> = self
            .authors
            .keys()
            .map(std::string::String::as_str)
            .collect();
        let from_cli: HashSet<_> = authors_initials.into_iter().collect();
        from_cli
            .difference(&configured)
            .into_iter()
            .copied()
            .collect()
    }

    /// Create a new author collection
    #[must_use]
    pub fn new(authors: BTreeMap<String, Author>) -> Self {
        Self { authors }
    }

    /// Get some authors by their initials
    #[must_use]
    pub fn get(&self, author_initials: &[&str]) -> Vec<&Author> {
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
        Self::new(store)
    }
}

impl IntoIterator for Authors {
    type IntoIter = IntoIter<String, Author>;
    type Item = (String, Author);

    fn into_iter(self) -> Self::IntoIter {
        self.authors.into_iter()
    }
}

impl TryFrom<&str> for Authors {
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

impl TryFrom<Authors> for String {
    type Error = SerializeAuthorsError;

    fn try_from(value: Authors) -> Result<Self, Self::Error> {
        toml::to_string(&value.authors).map_err(SerializeAuthorsError)
    }
}
