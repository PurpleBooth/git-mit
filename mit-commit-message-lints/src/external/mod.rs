pub use self::{
    config::read_toml,
    git2::Git2,
    in_memory::InMemory,
    vcs::{Error, Vcs},
};

mod config;
mod git2;
mod in_memory;
mod vcs;
