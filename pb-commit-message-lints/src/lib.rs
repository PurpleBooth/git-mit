mod author;
mod config;
mod lints;

#[macro_use]
extern crate partial_application;

pub use crate::config::{
    Git2Vcs as Git2VcsConfig,
    InMemoryVcs as InMemoryVcsConfig,
    Vcs as VcsConfig,
};

pub use crate::lints::{
    get_lint_configuration,
    has_duplicated_trailers,
    has_missing_pivotal_tracker_id,
    Lints,
};

pub use crate::author::{get_coauthor_configuration, Author};
