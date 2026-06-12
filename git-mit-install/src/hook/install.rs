use std::path::{Path, PathBuf};

use miette::{IntoDiagnostic, Result};

use crate::errors::GitMitInstallError;
pub fn link(hook_path: &Path, hook_name: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    let suffix = ".exe";
    #[cfg(not(target_os = "windows"))]
    let suffix = "";
    let binary_path = which::which(format!("mit-{hook_name}{suffix}")).into_diagnostic()?;
    let binary_path = binary_path.canonicalize().into_diagnostic()?;
    let install_path = hook_path.join(format!("{hook_name}{suffix}"));
    if let Ok(existing_hook_path) = install_path.canonicalize() {
        if existing_hook_path == binary_path {
            return Ok(());
        }
    }

    if install_path.exists() {
        if let Ok(dest) = install_path.canonicalize() {
            return Err(GitMitInstallError::ExistingSymlink(
                install_path.to_string_lossy().to_string(),
                dest.to_string_lossy().to_string(),
            )
            .into());
        }

        return Err(
            GitMitInstallError::ExistingHook(install_path.to_string_lossy().to_string()).into(),
        );
    }

    symlink(binary_path, install_path)?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn symlink(binary_path: PathBuf, install_path: PathBuf) -> Result<()> {
    std::os::unix::fs::symlink(binary_path, install_path).into_diagnostic()?;

    Ok(())
}

#[cfg(test)]
#[cfg(not(target_os = "windows"))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn link_detects_existing_correct_symlink_and_rejects_regular_file() {
        let temp = std::env::temp_dir().join(format!(
            "git-mit-install-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let hook_dir = temp.join("hooks");
        let bin_dir = temp.join("bin");
        std::fs::create_dir_all(&hook_dir).unwrap();
        std::fs::create_dir_all(&bin_dir).unwrap();

        let binary = bin_dir.join("mit-pre-commit");
        std::fs::File::create(&binary).unwrap();
        let mut perms = std::fs::metadata(&binary).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&binary, perms).unwrap();

        let old_path = std::env::var("PATH").unwrap_or_default();
        std::env::set_var("PATH", format!("{}:{}", bin_dir.display(), old_path));

        // First install creates the symlink
        link(&hook_dir, "pre-commit").unwrap();

        // Second install should succeed because symlink already points to correct binary
        let result = link(&hook_dir, "pre-commit");
        assert!(
            result.is_ok(),
            "Expected Ok(()) when symlink already points to correct binary, got {:?}",
            result
        );

        // Replace symlink with a regular file
        std::fs::remove_file(hook_dir.join("pre-commit")).unwrap();
        std::fs::File::create(hook_dir.join("pre-commit")).unwrap();

        let result = link(&hook_dir, "pre-commit");
        assert!(
            result.is_err(),
            "Expected error when regular file exists at install path, got Ok(())"
        );

        std::env::set_var("PATH", old_path);
        let _ = std::fs::remove_dir_all(&temp);
    }
}

#[cfg(target_os = "windows")]
fn symlink(binary_path: PathBuf, install_path: PathBuf) -> Result<()> {
    std::os::windows::fs::symlink_file(binary_path, install_path).into_diagnostic()?;

    Ok(())
}
