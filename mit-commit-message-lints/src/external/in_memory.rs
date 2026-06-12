use std::{collections::BTreeMap, convert::TryFrom};

use glob::Pattern;
use miette::{miette, IntoDiagnostic, Report, Result};

use crate::{
    external::{vcs::RepoState, Vcs},
    mit::{Author, Authors},
};

/// An in memory vcs implementation
///
/// Mostly used for testing
#[derive(Debug)]
pub struct InMemory<'a> {
    store: &'a mut BTreeMap<String, String>,
}

impl InMemory<'_> {
    /// Create a new in memory vcs
    #[must_use]
    pub const fn new(store: &mut BTreeMap<String, String>) -> InMemory<'_> {
        InMemory { store }
    }
}

impl Vcs for InMemory<'_> {
    fn entries(&self, glob: Option<&str>) -> Result<Vec<String>> {
        let compiled_glob = glob.map(Pattern::new).transpose().into_diagnostic()?;

        Ok(self
            .store
            .keys()
            .filter(|key| {
                compiled_glob
                    .as_ref()
                    .is_none_or(|pattern| pattern.matches(key))
            })
            .cloned()
            .collect())
    }

    fn get_bool(&self, name: &str) -> Result<Option<bool>> {
        match self.store.get(name) {
            None => Ok(None),
            Some(raw_value) => Ok(Some(raw_value.parse().into_diagnostic()?)),
        }
    }

    fn get_str(&self, name: &str) -> Result<Option<&str>> {
        Ok(self.store.get(name).map(String::as_str))
    }

    fn get_i64(&self, name: &str) -> Result<Option<i64>> {
        match self.store.get(name) {
            None => Ok(None),
            Some(raw_value) => Ok(Some(raw_value.parse().into_diagnostic()?)),
        }
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<()> {
        self.store.insert(name.into(), value.into());
        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<()> {
        self.store.insert(name.into(), format!("{value}"));
        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<()> {
        self.store.remove(name);
        Ok(())
    }

    fn state(&self) -> Option<RepoState> {
        None
    }
}

impl TryFrom<&'_ InMemory<'_>> for Authors<'_> {
    type Error = Report;

    fn try_from(vcs: &'_ InMemory<'_>) -> Result<Self, Self::Error> {
        let raw_entries: BTreeMap<String, BTreeMap<String, String>> = vcs
            .entries(Some("mit.author.config.*"))?
            .iter()
            .map(|key| (key, key.trim_start_matches("mit.author.config.")))
            .map(|(key, parts)| (key, parts.split_terminator('.').collect::<Vec<_>>()))
            .try_fold::<_, _, Result<_, Self::Error>>(
                BTreeMap::new(),
                |mut acc, (key, fragments)| {
                    let mut fragment_iterator = fragments.iter();
                    let initial = String::from(
                        *fragment_iterator
                            .next()
                            .ok_or_else(|| miette!("Malformed config key: {key}"))?,
                    );
                    let part = String::from(
                        *fragment_iterator
                            .next()
                            .ok_or_else(|| miette!("Malformed config key: {key}"))?,
                    );

                    let mut existing: BTreeMap<String, String> =
                        acc.get(&initial).cloned().unwrap_or_default();
                    existing.insert(part, String::from(vcs.get_str(key)?.unwrap()));

                    acc.insert(initial, existing);
                    Ok(acc)
                },
            )?;

        Ok(Self::new(
            raw_entries
                .iter()
                .filter_map(|(key, cfg)| {
                    let name = cfg.get("name").cloned();
                    let email = cfg.get("email").cloned();
                    let signingkey: Option<String> = cfg.get("signingkey").cloned();

                    match (name, email, signingkey) {
                        (Some(name), Some(email), None) => {
                            Some((key, Author::new(name.into(), email.into(), None)))
                        }
                        (Some(name), Some(email), Some(signingkey)) => Some((
                            key,
                            Author::new(name.into(), email.into(), Some(signingkey.into())),
                        )),
                        _ => None,
                    }
                })
                .map(|(key, value): (&String, Author<'_>)| (key.clone(), value))
                .collect(),
        ))
    }
}
