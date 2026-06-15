//! Resolving the commit-message file path
//!
//! Git passes the path to the commit message file as the first positional
//! argument to the `commit-msg` and `prepare-commit-msg` hooks. Hook
//! managers such as [lefthook](https://github.com/evilmartians/lefthook)
//! intercept these hooks but do not always forward that positional argument
//! to the commands they run. When the argument is missing we fall back to
//! the canonical location git writes the draft message to:
//! `<gitdir>/COMMIT_EDITMSG`.

use std::path::{Path, PathBuf};

use git2::Repository;
use miette::{IntoDiagnostic, Result};

/// Resolve the path to the file containing the commit log message.
///
/// If `provided` is `Some` it is returned unchanged — this is the normal
/// path when git invokes the hook directly.
///
/// If `provided` is `None` the git repository is discovered starting from
/// `current_dir` and the path to `COMMIT_EDITMSG` inside the git directory
/// is returned instead.
///
/// # Errors
///
/// If no path was provided and no git repository can be discovered from
/// `current_dir`.
pub fn resolve_commit_message_path(
    provided: Option<PathBuf>,
    current_dir: &Path,
) -> Result<PathBuf> {
    if let Some(path) = provided {
        Ok(path)
    } else {
        let repo = Repository::discover(current_dir).into_diagnostic()?;
        Ok(repo.path().join("COMMIT_EDITMSG"))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use super::resolve_commit_message_path;
    use git2::Repository;
    use tempfile::TempDir;

    #[test]
    fn returns_provided_path_unchanged() {
        let provided = PathBuf::from("/some/arbitrary/path");
        let result = resolve_commit_message_path(Some(provided.clone()), Path::new("/tmp"));
        assert_eq!(
            result.unwrap(),
            provided,
            "Expected the provided path to be returned unchanged"
        );
    }

    #[test]
    fn defaults_to_commit_editmsg_when_not_provided() {
        let temp = TempDir::new().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        let expected = repo.path().join("COMMIT_EDITMSG");

        let result = resolve_commit_message_path(None, temp.path());

        assert_eq!(
            result.unwrap(),
            expected,
            "Expected the default COMMIT_EDITMSG path when no path is provided"
        );
    }

    #[test]
    fn discovers_repo_from_subdirectory() {
        let temp = TempDir::new().unwrap();
        let repo = Repository::init(temp.path()).unwrap();
        let expected = repo.path().join("COMMIT_EDITMSG");

        let subdir = temp.path().join("nested/deep");
        fs::create_dir_all(&subdir).unwrap();

        let result = resolve_commit_message_path(None, &subdir);

        assert_eq!(
            result.unwrap(),
            expected,
            "Expected the COMMIT_EDITMSG path to be discovered from a subdirectory"
        );
    }

    #[test]
    fn errors_when_not_in_git_repo_and_not_provided() {
        let temp = TempDir::new().unwrap();
        let result = resolve_commit_message_path(None, temp.path());
        assert!(
            result.is_err(),
            "Expected an error when not in a git repo and no path is provided"
        );
    }
}
