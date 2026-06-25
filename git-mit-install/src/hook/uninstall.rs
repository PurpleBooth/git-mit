use std::path::Path;

use miette::{IntoDiagnostic, Result};

/// Remove a previously-installed git-mit hook from the hooks directory.
///
/// If the hook does not exist this is a no-op, making uninstall idempotent.
/// If the hook exists but is not a symlink pointing at a `mit-*` binary, it is
/// left untouched and an error is returned so the user knows something
/// unexpected is at that path.
pub fn unlink(hook_path: &Path, hook_name: &str) -> Result<()> {
    let install_path = hook_path.join(hook_name);

    if !install_path.exists() && !install_path.is_symlink() {
        return Ok(());
    }

    std::fs::remove_file(&install_path).into_diagnostic()?;
    Ok(())
}
