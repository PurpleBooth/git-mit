pub use cmd::lint;
pub use cmd::set_status;
pub use cmd::SetStatusError;
pub use lib::Error;
pub use lib::Lint;
pub use lib::LintError;
pub use lib::Lints;
pub use lib::Problem;

mod checks;
mod cmd;
mod lib;
