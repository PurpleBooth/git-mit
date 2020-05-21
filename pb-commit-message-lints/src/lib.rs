mod author;
mod config;
mod lints;

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

pub use crate::author::{get_author_configuration, Author};
