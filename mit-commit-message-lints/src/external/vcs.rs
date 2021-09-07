use std::convert::Infallible;

use thiserror::Error;

pub trait Vcs {
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a bool
    fn entries(&self, glob: Option<&str>) -> Result<Vec<String>, Error>;
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a bool
    fn get_bool(&self, name: &str) -> Result<Option<bool>, Error>;
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a &str
    fn get_str(&self, name: &str) -> Result<Option<&str>, Error>;
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a i64
    fn get_i64(&self, name: &str) -> Result<Option<i64>, Error>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_str(&mut self, name: &str, value: &str) -> Result<(), Error>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_i64(&mut self, name: &str, value: i64) -> Result<(), Error>;
    /// # Errors
    ///
    /// If the config fails to write
    fn remove(&mut self, name: &str) -> Result<(), Error>;
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to interact with git repository: {0}")]
    Git2Io(#[from] git2::Error),
    #[error("failed to read int from in memory datastore: {0}")]
    InMemoryParseInt(#[from] std::num::ParseIntError),
    #[error("failed to read bool from in memory datastore: {0}")]
    InMemoryParseBool(#[from] std::str::ParseBoolError),
    #[error("failed to read git-mit config")]
    Io(#[from] std::io::Error),
    #[error("failed to parse glob {0}")]
    Glob(#[from] glob::PatternError),
    #[error("{0}")]
    Infallible(#[from] Infallible),
}
