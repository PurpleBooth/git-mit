use std::convert::Infallible;

use miette::{Diagnostic, Result};
use thiserror::Error;

/// A wrapper around accessing different values from a VCS config
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
    /// If we can't read the config, or it's not parsable into an i64
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

    /// The state of the repository currently
    ///
    /// None if there is no repository, and we only have config
    fn state(&self) -> Option<RepoState>;
}

/// State of the repository
#[derive(Debug, Copy, Clone)]
pub enum RepoState {
    /// No other state is in progress
    Clean,
    /// Merging
    Merge,
    /// Reverting commit
    Revert,
    /// Reverting the sequence of commits
    RevertSequence,
    /// Cherry-picking commit
    CherryPick,
    /// Cherry-picking sequence of commits
    CherryPickSequence,
    /// git-bisect is in progress
    Bisect,
    /// Repository is in rebase
    Rebase,
    /// Repository is in interactive rebase
    RebaseInteractive,
    /// Repository is applying rebase merge
    RebaseMerge,
    /// Repository is applying mailbox
    ApplyMailbox,
    /// Repository is applying a mailbox patch or rebasing
    ApplyMailboxOrRebase,
}

/// Errors relating to different VCS implementations
#[derive(Error, Debug, Diagnostic)]
pub enum Error {
    /// Libgit2 sourced errors
    #[error("failed to interact with git repository: {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::git2_io)
    )]
    Git2Io(git2::Error),
    /// Failed to parse an int from the in-memory vcs
    #[error("failed to read int from in memory datastore: {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::in_memory_parse_int)
    )]
    InMemoryParseInt(std::num::ParseIntError),
    /// Failed to parse a bool from the in-memory vcs
    #[error("failed to read bool from in memory datastore: {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::in_memory_parse_bool)
    )]
    InMemoryParseBool(std::str::ParseBoolError),
    /// IO Failure when accessing VCS
    #[error("failed to read git-mit config")]
    #[diagnostic(url(docsrs), code(mit_commit_message_lints::external::vcs::error::io))]
    Io(std::io::Error),
    /// Failure to glob the config key correctly
    #[error("failed to parse glob {0}")]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::glob)
    )]
    Glob(glob::PatternError),
    /// Infallible
    #[error(transparent)]
    #[diagnostic(
        url(docsrs),
        code(mit_commit_message_lints::external::vcs::error::infallible)
    )]
    Infallible(Infallible),
}
