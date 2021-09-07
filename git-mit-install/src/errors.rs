use std::io;

#[derive(thiserror::Error, Debug)]
pub enum GitMitInstallError {
    #[error("failed install hook")]
    ExistingHook,
    #[error("failed to find git repository: {0}")]
    Git2(#[from] git2::Error),
    #[error("failed to install hooks: {0}")]
    Io(#[from] io::Error),
}
