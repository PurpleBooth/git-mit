use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("The details of the author of this commit are stale")]
#[diagnostic(
url(docsrs),
code(mit_pre_commit::errors::stale_author_error),
help("Can you confirm who's currently coding? It's nice to get and give the right credit. You can fix this by running `git mit` then the initials of whoever is coding for example: `git mit bt` or `git mit bt se`"),
)]
pub struct StaleAuthorError {}
