//! Mit commands
const CONFIG_KEY_EXPIRES: &str = "mit.author.expires";
const CONFIG_KEY_ROTATION: &str = "mit.author.rotate";

pub(crate) mod get_authors;
#[cfg(test)]
mod get_authors_test;
pub(crate) mod get_commit_coauthor_configuration;
#[cfg(test)]
pub(crate) mod get_commit_coauthor_configuration_test;
pub(crate) mod set_commit_authors;
pub(crate) mod set_config_authors;

pub mod errors;
pub mod get_config_non_clean_behaviour;

/// Configuration for rotating primary author across commits
///
/// When rotation is enabled, the primary author (user.name/user.email)
/// rotates among the configured authors with each commit.
pub mod get_config_rotation;
#[cfg(test)]
mod get_config_rotation_test;
/// Rotate the primary author among configured authors
pub mod rotate_authors;
#[cfg(test)]
mod set_commit_authors_test;
#[cfg(test)]
mod set_config_authors_test;
pub mod set_config_non_clean_behaviour;
/// Configuration for rotating primary author across commits
pub mod set_config_rotation;
#[cfg(test)]
mod set_config_rotation_test;
mod vcs;
