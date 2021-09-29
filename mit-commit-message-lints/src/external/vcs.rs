use std::convert::Infallible;

use miette::{Diagnostic, Result};
use thiserror::Error;
pub trait Vcs {
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a bool
    fn entries(&self, glob: Option<&str>) -> Result<Vec<String>>;
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a bool
    fn get_bool(&self, name: &str) -> Result<Option<bool>>;
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a &str
    fn get_str(&self, name: &str) -> Result<Option<&str>>;
    /// # Errors
    ///
    /// If we can't read the config, or it's not parsable into a i64
    fn get_i64(&self, name: &str) -> Result<Option<i64>>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_str(&mut self, name: &str, value: &str) -> Result<()>;
    /// # Errors
    ///
    /// If the config fails to write
    fn set_i64(&mut self, name: &str, value: i64) -> Result<()>;
    /// # Errors
    ///
    /// If the config fails to write
    fn remove(&mut self, name: &str) -> Result<()>;
}

#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    #[error("failed to interact with git repository: {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::git2_io)
    )]
    Git2Io(git2::Error),
    #[error("failed to read int from in memory datastore: {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::in_memory_parse_int)
    )]
    InMemoryParseInt(std::num::ParseIntError),
    #[error("failed to read bool from in memory datastore: {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::in_memory_parse_bool)
    )]
    InMemoryParseBool(std::str::ParseBoolError),
    #[error("failed to read git-mit config")]
    #[diagnostic(url(docsrs), code(mit_commit_message_lints::external::vcs::error::io))]
    Io(std::io::Error),
    #[error("failed to parse glob {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::glob)
    )]
    Glob(glob::PatternError),
    #[error(transparent)]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::infallible)
    )]
    Infallible(Infallible),
}
