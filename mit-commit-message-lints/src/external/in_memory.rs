use crate::external::{Error, Vcs};
use crate::mit::{Author, Authors};
use glob::Pattern;
use std::convert::TryFrom;
use std::{collections::BTreeMap, string::String};

pub struct InMemory<'a> {
    store: &'a mut BTreeMap<String, String>,
}

impl InMemory<'_> {
    #[must_use]
    pub fn new(store: &mut BTreeMap<String, String>) -> InMemory {
        InMemory { store }
    }
}

impl Vcs for InMemory<'_> {
    fn entries(&self, glob: Option<&str>) -> Result<Vec<String>, Error> {
        let mut keys: Vec<String> = self.store.keys().map(String::from).collect();

        if let Some(pattern) = glob {
            let compiled_glob = glob::Pattern::new(pattern)?;

            keys = keys
                .into_iter()
                .filter(|value| Pattern::matches(&compiled_glob, value))
                .collect();
        }

        Ok(keys)
    }

    fn get_bool(&self, name: &str) -> Result<Option<bool>, Error> {
        match self.store.get(name) {
            None => Ok(None),
            Some(raw_value) => Ok(Some(raw_value.parse().map_err(Error::from)?)),
        }
    }

    fn get_str(&self, name: &str) -> Result<Option<&str>, Error> {
        Ok(self.store.get(name).map(String::as_str))
    }

    fn get_i64(&self, name: &str) -> Result<Option<i64>, Error> {
        match self.store.get(name) {
            None => Ok(None),
            Some(raw_value) => Ok(Some(raw_value.parse().map_err(Error::from)?)),
        }
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Error> {
        self.store.insert(name.into(), value.into());
        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Error> {
        self.store.insert(name.into(), format!("{}", value));
        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<(), Error> {
        self.store.remove(name);
        Ok(())
    }
}

impl TryFrom<&'_ InMemory<'_>> for Authors {
    type Error = Error;

    fn try_from(vcs: &'_ InMemory) -> Result<Self, Self::Error> {
        let raw_entries: BTreeMap<String, BTreeMap<String, String>> = vcs
            .entries(Some("mit.author.config.*"))?
            .iter()
            .map(|key| (key, key.trim_start_matches("mit.author.config.")))
            .map(|(key, parts)| (key, parts.split_terminator('.').collect::<Vec<_>>()))
            .try_fold::<_, _, Result<_, Self::Error>>(
                BTreeMap::new(),
                |mut acc, (key, fragments)| {
                    let mut fragment_iterator = fragments.iter();
                    let initial = String::from(*fragment_iterator.next().unwrap());
                    let part = String::from(*fragment_iterator.next().unwrap());

                    let mut existing: BTreeMap<String, String> =
                        acc.get(&initial).map(BTreeMap::clone).unwrap_or_default();
                    existing.insert(part, String::from(vcs.get_str(key)?.unwrap()));

                    acc.insert(initial, existing);
                    Ok(acc)
                },
            )?;

        Ok(Authors::new(
            raw_entries
                .iter()
                .filter_map(|(key, cfg)| {
                    let name = cfg.get("name").map(String::clone);
                    let email = cfg.get("email").map(String::clone);
                    let signingkey: Option<String> = cfg.get("signingkey").map(String::clone);

                    match (name, email, signingkey) {
                        (Some(name), Some(email), None) => {
                            Some((key, Author::new(&name, &email, None)))
                        }
                        (Some(name), Some(email), Some(signingkey)) => {
                            Some((key, Author::new(&name, &email, Some(&signingkey))))
                        }
                        _ => None,
                    }
                })
                .fold(
                    BTreeMap::new(),
                    |mut acc: BTreeMap<String, Author>, (key, value): (&String, Author)| {
                        acc.insert(key.clone(), value);
                        acc
                    },
                ),
        ))
    }
}
