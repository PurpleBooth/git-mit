use std::{collections::BTreeMap, convert::TryFrom, path::PathBuf};

use git2::{Config, Repository, RepositoryState};
use miette::{IntoDiagnostic, Report, Result, miette};

use crate::{
    external::{Vcs, vcs::RepoState},
    mit::{Author, Authors},
};

/// Libgit2 vcs implementation
#[allow(missing_debug_implementations)]
pub struct Git2 {
    config_snapshot: Config,
    config_live: Config,
    state: Option<RepositoryState>,
}

impl Git2 {
    /// # Errors
    ///
    /// If it can't open the git config in snapshot mode
    pub fn new(mut config: Config, state: Option<RepositoryState>) -> Result<Self> {
        Ok(Self {
            config_snapshot: config.snapshot().into_diagnostic()?,
            config_live: config,
            state,
        })
    }

    fn config_defined(&self, lint_name: &str) -> Result<bool> {
        Ok(self
            .config_snapshot
            .entries(Some(lint_name))
            .into_diagnostic()?
            .next()
            .is_some())
    }

    fn refresh_snapshot(&mut self) -> Result<()> {
        self.config_snapshot = self.config_live.snapshot().into_diagnostic()?;
        Ok(())
    }
}

impl Vcs for Git2 {
    fn entries(&self, glob: Option<&str>) -> Result<Vec<String>> {
        let mut entries = vec![];
        let mut item = self.config_snapshot.entries(glob).into_diagnostic()?;
        while let Some(entry) = item.next() {
            if let Ok(name) = entry.into_diagnostic()?.name() {
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
        self.refresh_snapshot()
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<()> {
        self.config_live.set_i64(name, value).into_diagnostic()?;
        self.refresh_snapshot()
    }

    fn remove(&mut self, name: &str) -> Result<()> {
        self.config_live.remove(name).into_diagnostic()?;
        self.refresh_snapshot()
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
        let (config, state) = Repository::discover(current_dir)
            .and_then(|repo| {
                let state = repo.state();
                repo.config().map(|config| (config, Some(state)))
            })
            .or_else(|_| Config::open_default().map(|config| (config, None)))
            .into_diagnostic()?;
        Self::new(config, state)
    }
}

/// Parse a config key like `mit.author.config.bt.email` into its
/// initial (`bt`) and part (`email`).
///
/// The part is always the last dot-separated fragment (one of `name`,
/// `email`, `signingkey`). Everything before it is the initial, which
/// may itself contain dots (e.g. `b.t`).
///
/// # Errors
///
/// If the key does not contain at least an initial and a part.
fn parse_initial_and_part(config_key: &str) -> Result<(String, String)> {
    let stripped = config_key.trim_start_matches("mit.author.config.");
    let fragments: Vec<&str> = stripped.split_terminator('.').collect();
    if fragments.len() < 2 {
        return Err(miette!("Malformed config key: {config_key}"));
    }
    let part = String::from(fragments[fragments.len() - 1]);
    let initial = fragments[..fragments.len() - 1].join(".");
    Ok((initial, part))
}

impl TryFrom<&'_ Git2> for Authors<'_> {
    type Error = Report;

    fn try_from(vcs: &'_ Git2) -> Result<Self, Self::Error> {
        let raw_entries: BTreeMap<String, BTreeMap<String, String>> = vcs
            .entries(Some("mit.author.config.*"))?
            .iter()
            .try_fold::<_, _, Result<_, Self::Error>>(BTreeMap::new(), |mut acc, key| {
                let (initial, part) = parse_initial_and_part(key)?;
                let mut existing: BTreeMap<String, String> =
                    acc.get(&initial).cloned().unwrap_or_default();
                existing.insert(part, String::from(vcs.get_str(key)?.unwrap()));

                acc.insert(initial, existing);
                Ok(acc)
            })?;

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

#[cfg(test)]
mod tests {
    use super::parse_initial_and_part;

    #[test]
    fn parses_simple_initials() {
        let (initial, part) =
            parse_initial_and_part("mit.author.config.bt.email").expect("should parse");
        assert_eq!(initial, "bt", "Expected initial to be 'bt'");
        assert_eq!(part, "email", "Expected part to be 'email'");
    }

    #[test]
    fn parses_initials_containing_dots() {
        let (initial, part) =
            parse_initial_and_part("mit.author.config.b.t.email").expect("should parse");
        assert_eq!(
            initial, "b.t",
            "Expected initial to be 'b.t' for dotted initials"
        );
        assert_eq!(part, "email", "Expected part to be 'email'");
    }

    #[test]
    fn parses_signingkey_with_dotted_initials() {
        let (initial, part) =
            parse_initial_and_part("mit.author.config.b.t.signingkey").expect("should parse");
        assert_eq!(
            initial, "b.t",
            "Expected initial to be 'b.t' for dotted initials"
        );
        assert_eq!(part, "signingkey", "Expected part to be 'signingkey'");
    }

    #[test]
    fn errors_on_malformed_key() {
        parse_initial_and_part("mit.author.config.bt").expect_err("should fail");
    }
}
