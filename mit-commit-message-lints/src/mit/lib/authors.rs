use std::{
    collections::{btree_map::IntoIter, BTreeMap, HashSet},
    convert::TryFrom,
};

use crate::mit::lib::{
    author::Author,
    errors::{DeserializeAuthorsError, SerializeAuthorsError},
};

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Authors {
    pub authors: BTreeMap<String, Author>,
}

impl Authors {
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

    #[must_use]
    pub fn new(authors: BTreeMap<String, Author>) -> Self {
        Self { authors }
    }

    #[must_use]
    pub fn get(&self, author_initials: &[&str]) -> Vec<&Author> {
        author_initials
            .iter()
            .filter_map(|initial| self.authors.get(*initial))
            .collect()
    }

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
