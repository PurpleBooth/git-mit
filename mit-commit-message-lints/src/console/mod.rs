pub(crate) mod exit;
pub mod style;

pub use exit::initial_not_matched_to_author as exit_initial_not_matched_to_author;
pub use exit::lint_problem as exit_lint_problem;
pub use exit::stale_author as exit_stale_author;
pub use exit::unparsable_author as exit_unparsable_author;
pub use exit::Code as ExitCode;
pub use style::to_be_piped;
