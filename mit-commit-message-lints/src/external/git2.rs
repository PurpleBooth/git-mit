use std::{collections::BTreeMap, convert::TryFrom, path::PathBuf};

use git2::{Config, Repository, RepositoryState};
use miette::{IntoDiagnostic, Report, Result};

use crate::{
    external::{vcs::RepoState, Vcs},
    mit::{Author, Authors},
};

/// Libgit2 vcs implementation
#[allow(missing_debug_implementations)]
pub struct Git2 {
    config_snapshot: git2::Config,
    config_live: git2::Config,
    state: Option<git2::RepositoryState>,
}

impl Git2 {
    /// # Panics
    ///
    /// Will panic if it can't open the git config in snapshot mode
    #[must_use]
    pub fn new(mut config: git2::Config, state: Option<git2::RepositoryState>) -> Self {
        Self {
            config_snapshot: config.snapshot().unwrap(),
            config_live: config,
            state,
        }
    }

    fn config_defined(&self, lint_name: &str) -> Result<bool> {
        Ok(self
            .config_snapshot
            .entries(Some(lint_name))
            .into_diagnostic()?
            .next()
            .is_some())
    }
}

impl Vcs for Git2 {
    fn entries(&self, glob: Option<&str>) -> Result<Vec<String>> {
        let mut entries = vec![];
        let mut item = self.config_snapshot.entries(glob).into_diagnostic()?;
        while let Some(entry) = item.next() {
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

    fn state(&self) -> Option<RepoState> {
        match self.state {
            None => None,
            Some(RepositoryState::ApplyMailbox) => Some(RepoState::ApplyMailbox),
            Some(RepositoryState::Clean) => Some(RepoState::Clean),
            Some(RepositoryState::Merge) => Some(RepoState::Merge),
            Some(RepositoryState::Revert) => Some(RepoState::Revert),
            Some(RepositoryState::RevertSequence) => Some(RepoState::RevertSequence),
            Some(RepositoryState::CherryPick) => Some(RepoState::CherryPick),
            Some(RepositoryState::CherryPickSequence) => Some(RepoState::CherryPickSequence),
            Some(RepositoryState::Bisect) => Some(RepoState::Bisect),
            Some(RepositoryState::Rebase) => Some(RepoState::Rebase),
            Some(RepositoryState::RebaseInteractive) => Some(RepoState::RebaseInteractive),
            Some(RepositoryState::RebaseMerge) => Some(RepoState::RebaseMerge),
            Some(RepositoryState::ApplyMailboxOrRebase) => Some(RepoState::ApplyMailboxOrRebase),
        }
    }
}

impl TryFrom<PathBuf> for Git2 {
    type Error = Report;

    fn try_from(current_dir: PathBuf) -> Result<Self, Self::Error> {
        Repository::discover(current_dir)
            .and_then(|repo| {
                let state = repo.state();
                repo.config().map(|config| (config, Some(state)))
            })
            .or_else(|_| (Config::open_default().map(|config| (config, None))))
            .map(|(config, state)| Self::new(config, state))
            .into_diagnostic()
    }
}

impl TryFrom<&'_ Git2> for Authors<'_> {
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
                .fold(
                    BTreeMap::new(),
                    |mut acc: BTreeMap<String, Author<'_>>, (key, value): (&String, Author<'_>)| {
                        acc.insert(key.clone(), value);
                        acc
                    },
                ),
        ))
    }
}
