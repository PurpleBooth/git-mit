pub use cmd::{lint, set_status, SetStatusError};
pub use lib::{Error, Lint, LintError, Lints, Problem};

mod checks;
mod cmd;
mod lib;
