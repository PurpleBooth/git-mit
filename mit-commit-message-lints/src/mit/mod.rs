pub use cmd::errors::Error as VcsError;
pub use cmd::get_commit_coauthor_configuration::get_commit_coauthor_configuration;
pub use cmd::set_commit_authors::set_commit_authors;
pub use cmd::set_config_authors::set_config_authors;
pub use lib::author::Author;
pub use lib::authors::Authors;
pub use lib::authors::ConfigParseError as AuthorConfigParseError;

pub mod cmd;
pub mod lib;
