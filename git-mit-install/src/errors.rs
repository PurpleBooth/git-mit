use miette::Diagnostic;
#[derive(thiserror::Error, Debug, Diagnostic)]
pub enum GitMitInstallError {
    #[error("failed to install hook")]
    #[diagnostic(
        code(git_mit_install::errors::git_mit_install_error::existing_hook),
        url(docsrs),
        help("{0} already exists, you need to remove this before continuing")
    )]
    ExistingHook(String),
    #[error("failed to install hook")]
    #[diagnostic(
        code(git_mit_install::errors::git_mit_install_error::existing_symlink),
        url(docsrs),
        help("{0} already exists, you need to remove this before continuing, looks like it's a symlink to {1}")
    )]
    ExistingSymlink(String, String),
}
