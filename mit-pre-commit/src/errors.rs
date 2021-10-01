use std::fmt::{Debug, Display};

use chrono::{DateTime, Local, Utc};
use miette::{Diagnostic, LabeledSpan, SourceCode};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("The details of the author of this commit are stale")]
pub struct StaleAuthorError {
    source_code: String,
    date: DateTime<Utc>,
}

impl StaleAuthorError {
    pub(crate) fn new(last_updated: DateTime<Utc>) -> StaleAuthorError {
        StaleAuthorError {
            source_code: DateTime::<Local>::from(last_updated).to_string(),
            date: last_updated,
        }
    }
}

impl Diagnostic for StaleAuthorError {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new("mit_pre_commit::errors::stale_author_error"))
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new("Can you confirm who's currently coding? It's nice to get and give the right credit. You can fix this by running `git mit` then the initials of whoever is coding for example: `git mit bt` or `git mit bt se`"))
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.source_code)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        Some(Box::new(
            vec![LabeledSpan::new(
                Some("The previously set authors expired at this time".to_string()),
                0_usize,
                self.source_code.len(),
            )]
            .into_iter(),
        ))
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("No authors set")]
#[diagnostic(
code(mit_pre_commit::errors::stale_author_error),
help("Can you set who's currently coding? It's nice to get and give the right credit. You can fix this by running `git mit` then the initials of whoever is coding for example: `git mit bt` or `git mit bt se`")
)]
pub struct NoAuthorError {}
