use std::{
    collections::{btree_map::IntoIter, BTreeMap, HashSet},
    convert::TryFrom,
};

use miette::{Diagnostic, SourceOffset, SourceSpan};
use thiserror::Error;

use crate::mit::lib::author::Author;

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
    type Error = SerialiseAuthorsError;

    fn try_from(input: &str) -> std::result::Result<Self, Self::Error> {
        serde_yaml::from_str(input)
            .or_else(|yaml_error| {
                toml::from_str(input).map_err(|toml_error| SerialiseAuthorsError {
                    src: input.to_string(),
                    toml_span: (span_from_toml_err(&toml_error, input), 0).into(),
                    yaml_span: (span_from_yaml_err(&yaml_error, input), 0).into(),
                    yaml_message: "".to_string(),
                    toml_message: "".to_string(),
                })
            })
            .map(Self::new)
    }
}

fn span_from_toml_err(err: &toml::de::Error, input: &str) -> usize {
    err.line_col()
        .map_or(SourceOffset::from(0), |(line, col)| {
            SourceOffset::from_location(input, line, col)
        })
        .offset()
}

fn span_from_yaml_err(err: &serde_yaml::Error, input: &str) -> usize {
    err.location()
        .map_or(SourceOffset::from(0), |location| {
            SourceOffset::from_location(input, location.line(), location.column())
        })
        .offset()
}

#[derive(Error, Debug, Diagnostic)]
#[error("could not parse author configuration")]
#[diagnostic(
code(mit_commit_message_lints::mit::lib::authors::serialise_authors_error),
help("`git mit-config mit example` can show you an example of what it should look like, or you can generate one using `git mit-config mit generate` after setting up some authors with `git mit-config mit set`"),
)]
pub struct SerialiseAuthorsError {
    #[source_code]
    src: String,
    #[label("invalid in toml: {toml_message}")]
    toml_span: SourceSpan,
    #[label("invalid in yaml: {yaml_message}")]
    yaml_span: SourceSpan,

    yaml_message: String,
    toml_message: String,
}

impl From<Authors> for String {
    fn from(value: Authors) -> Self {
        toml::to_string(&value.authors).unwrap()
    }
}
