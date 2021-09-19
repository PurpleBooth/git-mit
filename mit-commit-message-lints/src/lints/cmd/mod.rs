pub use read_lint_config::{read_from_toml_or_else_vcs, Error as ReadFromTomlOrElseVcsError};
pub use set_status::{set_status, Error as SetStatusError};

mod read_lint_config;
mod set_status;
