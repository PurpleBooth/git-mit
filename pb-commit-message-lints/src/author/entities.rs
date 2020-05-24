use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Author {
    name: String,
    email: String,
}

impl Author {
    #[must_use]
    pub fn new(name: &str, email: &str) -> Author {
        Author {
            name: name.into(),
            email: email.into(),
        }
    }

    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub fn email(&self) -> String {
        self.email.clone()
    }
}

#[cfg(test)]
mod tests_author {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn has_an_author() {
        let author = Author::new("The Name", "email@example.com");

        assert_eq!(author.name(), "The Name");
        assert_eq!(author.email(), "email@example.com");
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Authors {
    pub authors: HashMap<String, Author>,
}

impl Authors {
    #[must_use]
    pub fn new(authors: HashMap<String, Author>) -> Authors {
        Authors { authors }
    }

    #[must_use]
    pub fn get(&self, author_initials: &[&str]) -> Vec<Option<&Author>> {
        author_initials
            .iter()
            .map(|initial| self.authors.get(*initial))
            .collect()
    }
}

#[cfg(test)]
mod tests_authors {
    #![allow(clippy::wildcard_imports)]

    use super::*;

    #[test]
    fn it_can_get_an_author_in_it() {
        let mut store = HashMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com"),
        );
        let actual_authors = Authors::new(store);

        assert_eq!(
            actual_authors.get(&["bt"]),
            vec![Some(&Author::new("Billie Thompson", "billie@example.com"))]
        )
    }

    #[test]
    fn i_can_get_multiple_authors_out_at_the_same_time() {
        let mut store: HashMap<String, Author> = HashMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com"),
        );
        store.insert(
            "se".into(),
            Author::new("Somebody Else", "somebody@example.com"),
        );
        let actual = Authors::new(store);

        assert_eq!(
            actual.get(&["bt"]),
            vec![Some(&Author::new("Billie Thompson", "billie@example.com"))]
        );
        assert_eq!(
            actual.get(&["se"]),
            vec![Some(&Author::new("Somebody Else", "somebody@example.com"))]
        )
    }
}
