pub use cmd::{
    get_authors::{get_authors, AuthorArgs, AuthorArgsScope},
    get_commit_coauthor_configuration::get_commit_coauthor_configuration,
    set_commit_authors::set_commit_authors,
    set_config_authors::set_config_authors,
};
pub use lib::{author::Author, author_state::AuthorState, authors::Authors};

pub mod cmd;
pub mod lib;
