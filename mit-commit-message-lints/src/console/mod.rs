pub(crate) mod exit;
pub mod style;

pub use exit::{
    initial_not_matched_to_author as exit_initial_not_matched_to_author,
    lint_problem as exit_lint_problem,
    stale_author as exit_stale_author,
    unparsable_author as exit_unparsable_author,
};
pub use style::to_be_piped;
