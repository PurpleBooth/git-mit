use std::path::Path;

use miette::{IntoDiagnostic, Result};

use crate::errors::GitMitInstallError;

/// Convert OS path separators to forward slashes so a path is valid inside a
/// `#!/bin/sh` wrapper on Windows. On Unix this is effectively a no-op.
fn path_forward_slashes(path: &Path) -> String {
    path.display().to_string().replace('\\', "/")
}

/// The exact content of the Windows hook wrapper: a POSIX shell script that
/// execs the resolved mit binary, forwarding all arguments.
fn wrapper_content(binary_path: &Path) -> String {
    let target = path_forward_slashes(binary_path);
    format!("#!/bin/sh\nexec \"{target}\" \"$@\"\n")
}

/// True iff the file at `install_path` contains exactly our wrapper for
/// `binary_path`. Used for idempotent re-install on Windows.
fn is_our_wrapper(install_path: &Path, binary_path: &Path) -> bool {
    std::fs::read_to_string(install_path)
        .is_ok_and(|content| content == wrapper_content(binary_path))
}

pub fn link(hook_path: &Path, hook_name: &str) -> Result<()> {
    let binary_path = which::which(binary_name(hook_name)).into_diagnostic()?;
    let binary_path = binary_path.canonicalize().into_diagnostic()?;
    let install_path = hook_path.join(install_name(hook_name));

    if already_installed(&install_path, &binary_path) {
        return Ok(());
    }

    if install_path.exists() {
        return Err(
            GitMitInstallError::ExistingHook(install_path.to_string_lossy().to_string()).into(),
        );
    }

    #[cfg(target_os = "windows")]
    write_wrapper(&binary_path, &install_path)?;
    #[cfg(not(target_os = "windows"))]
    symlink(&binary_path, &install_path)?;

    Ok(())
}

/// The mit binary to locate on `PATH`, e.g. `mit-pre-commit` (unix) or
/// `mit-pre-commit.exe` (windows).
fn binary_name(hook_name: &str) -> String {
    #[cfg(target_os = "windows")]
    let suffix = ".exe";
    #[cfg(not(target_os = "windows"))]
    let suffix = "";
    format!("mit-{hook_name}{suffix}")
}

/// The filename Git looks for in the hooks directory. On every platform this is
/// the bare hook name (`pre-commit`); Git's `find_hook` does not try extensions,
/// so a `.exe` suffix here would be silently ignored on Windows.
fn install_name(hook_name: &str) -> String {
    hook_name.to_string()
}

/// True iff the hook at `install_path` is already correctly installed for
/// `binary_path`. On unix a path resolving to the binary counts; on windows a
/// wrapper whose content matches ours counts. Both branches compile on every
/// platform so the windows logic is unit-testable on unix.
fn already_installed(install_path: &Path, binary_path: &Path) -> bool {
    if cfg!(target_os = "windows") {
        is_our_wrapper(install_path, binary_path)
    } else {
        install_path
            .canonicalize()
            .is_ok_and(|resolved| resolved == binary_path)
    }
}

#[cfg(not(target_os = "windows"))]
fn symlink(binary_path: &Path, install_path: &Path) -> Result<()> {
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
fn write_wrapper(binary_path: &Path, install_path: &Path) -> Result<()> {
    std::fs::write(install_path, wrapper_content(binary_path)).into_diagnostic()
}

#[cfg(test)]
mod pure_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn path_forward_slashes_converts_backslashes() {
        let p = path_forward_slashes(&PathBuf::from(r"C:\Users\git-mit\mit-pre-commit.exe"));
        assert_eq!(
            p, "C:/Users/git-mit/mit-pre-commit.exe",
            "backslashes must become forward slashes for use in a shell script"
        );
    }

    #[test]
    fn path_forward_slashes_leaves_unix_paths_unchanged() {
        let p = path_forward_slashes(&PathBuf::from("/usr/local/bin/mit-pre-commit"));
        assert_eq!(
            p, "/usr/local/bin/mit-pre-commit",
            "unix paths already use forward slashes"
        );
    }

    #[test]
    fn wrapper_content_targets_canonical_path_with_shebang() {
        let content = wrapper_content(&PathBuf::from(r"C:\Users\git-mit\mit-pre-commit.exe"));
        assert_eq!(
            content, "#!/bin/sh\nexec \"C:/Users/git-mit/mit-pre-commit.exe\" \"$@\"\n",
            "wrapper must be a sh script that execs the absolute, slash-normalised binary"
        );
    }

    #[test]
    fn is_our_wrapper_is_true_for_our_wrapper_content() {
        let temp = std::env::temp_dir().join(format!(
            "git-mit-install-pure-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&temp).unwrap();

        let binary = temp.join("mit-pre-commit");
        std::fs::write(&binary, b"binary-bytes").unwrap();
        let binary = binary.canonicalize().unwrap();

        let hook = temp.join("pre-commit");
        std::fs::write(&hook, wrapper_content(&binary)).unwrap();

        assert!(
            is_our_wrapper(&hook, &binary),
            "a file containing our exact wrapper content must count as already installed"
        );

        let _ = std::fs::remove_dir_all(&temp);
    }

    #[test]
    fn is_our_wrapper_is_false_for_foreign_file() {
        let temp = std::env::temp_dir().join(format!(
            "git-mit-install-pure-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&temp).unwrap();

        let binary = temp.join("mit-pre-commit");
        std::fs::write(&binary, b"binary-bytes").unwrap();
        let binary = binary.canonicalize().unwrap();

        let hook = temp.join("pre-commit");
        std::fs::write(&hook, b"#!/bin/sh\necho someone elses hook\n").unwrap();

        assert!(
            !is_our_wrapper(&hook, &binary),
            "a file with different content must not count as already installed"
        );

        let _ = std::fs::remove_dir_all(&temp);
    }
}

#[cfg(all(test, target_os = "windows"))]
mod windows_tests {
    use super::*;

    #[test]
    fn link_writes_runnable_wrapper_and_is_idempotent() {
        let temp = std::env::temp_dir().join(format!(
            "git-mit-install-win-test-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let hook_dir = temp.join("hooks");
        let bin_dir = temp.join("bin");
        std::fs::create_dir_all(&hook_dir).unwrap();
        std::fs::create_dir_all(&bin_dir).unwrap();

        // Place a mit-pre-commit.exe on PATH so `which` can resolve it.
        let binary = bin_dir.join("mit-pre-commit.exe");
        std::fs::write(&binary, b"placeholder").unwrap();
        let old_path = std::env::var("PATH").unwrap_or_default();
        std::env::set_var("PATH", format!("{};{old_path}", bin_dir.display()));

        // First install writes the wrapper at the bare hook name Git looks for.
        link(&hook_dir, "pre-commit").unwrap();
        let hook = hook_dir.join("pre-commit");
        assert!(
            hook.exists(),
            "hook must be written at its bare name for Git to find it"
        );
        assert!(
            !hook_dir.join("pre-commit.exe").exists(),
            "must not create the .exe-named hook that Git ignores"
        );

        let content = std::fs::read_to_string(&hook).unwrap();
        assert!(
            content.starts_with("#!/bin/sh\n"),
            "wrapper must be a sh script, got: {content:?}"
        );

        // Second install must be a no-op, not an error.
        let again = link(&hook_dir, "pre-commit");
        assert!(
            again.is_ok(),
            "re-install must be idempotent, got {again:?}"
        );

        std::env::set_var("PATH", old_path);
        let _ = std::fs::remove_dir_all(&temp);
    }
}
