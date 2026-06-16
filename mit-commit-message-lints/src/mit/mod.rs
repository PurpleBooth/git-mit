//! Interactions relating to reading and setting authors

pub use cmd::{
    get_authors::{AuthorArgs, GenericArgs, get_authors},
    get_commit_coauthor_configuration::get_commit_coauthor_configuration,
    get_config_rotation::get_config_rotation,
    rotate_authors::rotate_authors,
    set_commit_authors::set_commit_authors,
    set_config_authors::set_config_authors,
    set_config_rotation::set_config_rotation,
};
pub use lib::{
    author::Author, author_state::AuthorState, authors::Authors, rotation_option::RotationOption,
};

pub mod cmd;
pub mod lib;
