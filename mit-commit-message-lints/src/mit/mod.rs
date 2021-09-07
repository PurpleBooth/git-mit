pub use cmd::{
    errors::Error as VcsError,
    get_commit_coauthor_configuration::get_commit_coauthor_configuration,
    set_commit_authors::set_commit_authors,
    set_config_authors::set_config_authors,
};
pub use lib::{
    author::Author,
    authors::{Authors, ConfigParseError as AuthorConfigParseError},
};

pub mod cmd;
pub mod lib;
