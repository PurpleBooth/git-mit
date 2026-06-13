//! Implementations of VCS we can interact with

pub use self::{
    commit_message_path::resolve_commit_message_path,
    config::read_toml,
    git2::Git2,
    in_memory::InMemory,
    vcs::{Error, RepoState, Vcs},
};

mod commit_message_path;
mod config;
mod git2;
mod in_memory;
mod vcs;
