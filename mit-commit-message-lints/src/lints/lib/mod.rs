pub use error::Error;
pub use lint::Error as LintError;
pub use lint::Lint;
pub use lints::Error as LintsError;
pub use lints::Lints;
pub use problem::{Code, Problem};

mod error;
mod lint;
mod lints;
mod subject_line_ends_with_period;
mod subject_longer_than_72_characters;
mod subject_not_capitalized;
mod subject_not_seperate_from_body;

mod duplicate_trailers;
mod missing_github_id;
mod missing_jira_issue_key;
mod missing_pivotal_tracker_id;
mod problem;
