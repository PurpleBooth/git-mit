use miette::Diagnostic;
#[derive(thiserror::Error, Debug, Diagnostic)]
pub enum GitMitInstallError {
    #[error("failed to install hook")]
    #[diagnostic(
        url(docsrs),
        help("open `.git/hooks` and see if there's something conflicting there")
    )]
    ExistingHook,
}
