use mit_commit::CommitMessage;

use crate::lints::lib::{Lints, Problem};

#[must_use]
pub fn lint(commit_message: &CommitMessage, lints: Lints) -> Vec<Problem> {
    lints
        .into_iter()
        .flat_map(|lint| lint.lint(commit_message))
        .collect::<Vec<Problem>>()
}
