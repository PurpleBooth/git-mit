const CONFIG_KEY_EXPIRES: &str = "mit.author.expires";

pub(crate) mod get_authors;
pub(crate) mod get_commit_coauthor_configuration;
#[cfg(test)]
pub(crate) mod get_commit_coauthor_configuration_test;
pub(crate) mod set_commit_authors;
pub(crate) mod set_config_authors;

pub mod errors;
#[cfg(test)]
mod set_commit_authors_test;
#[cfg(test)]
mod set_config_authors_test;
mod vcs;
