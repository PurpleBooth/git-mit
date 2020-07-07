const CONFIG_KEY_EXPIRES: &str = "mit.author.expires";

pub(crate) mod get_commit_coauthor_configuration;
pub(crate) mod get_config_authors;
pub(crate) mod set_commit_authors;
pub(crate) mod set_config_authors;

pub mod errors;
mod vcs;
