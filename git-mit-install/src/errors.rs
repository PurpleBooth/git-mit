use miette::Diagnostic;
#[derive(thiserror::Error, Debug, Diagnostic)]
pub enum GitMitInstallError {
    #[error("failed to install hook")]
    #[diagnostic(
        code(git_mit_install::errors::git_mit_install_error::existing_hook),
        url(docsrs),
        help("open `.git/hooks` and see if there's something conflicting there")
    )]
    ExistingHook,
}
