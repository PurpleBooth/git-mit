use crate::external::{Error, Vcs};
use git2::{Config, Repository};
use serde::export::TryFrom;
use std::path::PathBuf;

pub struct Git2 {
    config_snapshot: git2::Config,
    config_live: git2::Config,
}

impl Git2 {
    #[must_use]
    pub fn new(mut config: git2::Config) -> Git2 {
        Git2 {
            config_snapshot: config.snapshot().unwrap(),
            config_live: config,
        }
    }

    fn config_defined(&self, lint_name: &str) -> Result<bool, Error> {
        self.config_snapshot
            .entries(Some(lint_name))
            .map(|entries| entries.count() > 0)
            .map_err(Error::from)
    }
}

impl Vcs for Git2 {
    fn get_bool(&self, name: &str) -> Result<Option<bool>, Error> {
        if self.config_defined(name)? {
            Ok(Some(self.config_snapshot.get_bool(name)?))
        } else {
            Ok(None)
        }
    }

    fn get_str(&self, name: &str) -> Result<Option<&str>, Error> {
        let defined = self.config_defined(name)?;

        if defined {
            self.config_snapshot
                .get_str(name)
                .map(Some)
                .map_err(Error::from)
        } else {
            Ok(None)
        }
    }

    fn get_i64(&self, name: &str) -> Result<Option<i64>, Error> {
        let defined = self.config_defined(name)?;

        if defined {
            self.config_snapshot
                .get_i64(name)
                .map(Some)
                .map_err(Error::from)
        } else {
            Ok(None)
        }
    }

    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Error> {
        self.config_live.set_str(name, value)?;

        let config = self.config_live.snapshot()?;

        self.config_snapshot = config;

        Ok(())
    }

    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Error> {
        self.config_live.set_i64(name, value)?;

        let config = self.config_live.snapshot()?;
        self.config_snapshot = config;

        Ok(())
    }

    fn remove(&mut self, name: &str) -> Result<(), Error> {
        self.config_live.remove(name)?;

        let config = self.config_live.snapshot()?;
        self.config_snapshot = config;

        Ok(())
    }
}

impl TryFrom<PathBuf> for Git2 {
    type Error = Error;

    fn try_from(current_dir: PathBuf) -> Result<Self, Self::Error> {
        Repository::discover(current_dir)
            .and_then(|x| x.config())
            .or_else(|_| Config::open_default())
            .map(Git2::new)
            .map_err(Error::from)
    }
}
