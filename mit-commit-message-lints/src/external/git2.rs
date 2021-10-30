use std::{collections::BTreeMap, convert::TryFrom, path::PathBuf};

use git2::{Config, Repository};
use miette::{IntoDiagnostic, Report, Result};

use crate::{
    external::Vcs,
    mit::{Author, Authors},
};

/// Libgit2 vcs implementation
#[allow(missing_debug_implementations)]
pub struct Git2 {
    config_snapshot: git2::Config,
    config_live: git2::Config,
}

impl Git2 {
    /// # Panics
    ///
    /// Will panic if it can't open the git config in snapshot mode
    #[must_use]
    pub fn new(mut config: git2::Config) -> Self {
        Self {
            config_snapshot: config.snapshot().unwrap(),
            config_live: config,
        }
    }

    fn config_defined(&self, lint_name: &str) -> Result<bool> {
        self.config_snapshot
            .entries(Some(lint_name))
            .map(|entries| entries.count() > 0)
            .into_diagnostic()
    }
}

impl Vcs for Git2 {
    fn entries(&self, glob: Option<&str>) -> Result<Vec<String>> {
        let mut entries = vec![];
        for entry in &self.config_snapshot.entries(glob).into_diagnostic()? {
            if let Some(name) = entry.into_diagnostic()?.name() {
                entries.push(name.into());
            }
        }

        Ok(entries)
    }

    fn get_bool(&self, name: &str) -> Result<Option<bool>> {
        if self.config_defined(name)? {
            Ok(Some(self.config_snapshot.get_bool(name).into_diagnostic()?))
        } else {
            Ok(None)
        }
    }

    fn get_str(&self, name: &str) -> Result<Option<&str>> {
        let defined = self.config_defined(name)?;

        if defined {
            self.config_snapshot
                .get_str(name)
                .map(Some)
                .into_diagnostic()
        } else {
            Ok(None)
        }
    }

    fn get_i64(&self, name: &str) -> Result<Option<i64>> {
        let defined = self.config_defined(name)?;

        if defined {
            self.config_snapshot
                .get_i64(name)
                .map(Some)
                .into_diagnostic()
        } else {
            Ok(None)
        }
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<()> {
        self.config_live.set_str(name, value).into_diagnostic()?;

        let config = self.config_live.snapshot().into_diagnostic()?;

        self.config_snapshot = config;

        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<()> {
        self.config_live.set_i64(name, value).into_diagnostic()?;

        let config = self.config_live.snapshot().into_diagnostic()?;
        self.config_snapshot = config;

        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<()> {
        self.config_live.remove(name).into_diagnostic()?;

        let config = self.config_live.snapshot().into_diagnostic()?;
        self.config_snapshot = config;

        Ok(())
    }
}

impl TryFrom<PathBuf> for Git2 {
    type Error = Report;

    fn try_from(current_dir: PathBuf) -> Result<Self, Self::Error> {
        Repository::discover(current_dir)
            .and_then(|x| x.config())
            .or_else(|_| Config::open_default())
            .map(Self::new)
            .into_diagnostic()
    }
}

impl TryFrom<&'_ Git2> for Authors {
    type Error = Report;

    fn try_from(vcs: &'_ Git2) -> Result<Self, Self::Error> {
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

        Ok(Self::new(
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
