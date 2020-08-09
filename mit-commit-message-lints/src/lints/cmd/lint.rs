use mit_commit::CommitMessage;

use crate::lints::lib::{Lints, Problem};
use rayon::prelude::*;

#[must_use]
pub fn lint(commit_message: &CommitMessage, lints: Lints) -> Vec<Problem> {
    lints
        .into_iter()
        .collect::<Vec<_>>()
        .into_par_iter()
        .flat_map(|lint| lint.lint(commit_message))
        .collect::<Vec<Problem>>()
}
