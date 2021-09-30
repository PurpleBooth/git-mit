use miette::{Diagnostic, SourceSpan};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug, Diagnostic)]
#[error("could not parse lint configuration")]
#[diagnostic(
    url(docsrs),
    code(mit_commit_message_lints::lints::cmd::read_lint_config::serialise_lint_error),
    help("you can generate an example using `git mit-config lint generate`")
)]
pub struct SerialiseLintError {
    #[source_code]
    pub(crate) src: String,
    #[label("invalid in toml: {message}")]
    pub(crate) span: SourceSpan,
    pub(crate) message: String,
}
