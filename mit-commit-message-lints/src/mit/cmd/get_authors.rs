use std::{
    convert::TryFrom,
    fs,
    path::PathBuf,
    process::{Command, Stdio},
};

use miette::{IntoDiagnostic, Result};

use crate::mit::Authors;

/// A generic structure to pass around details needed to get authors
#[derive(Debug, Clone)]
pub struct GenericArgs<'a> {
    /// Command to be executed
    pub author_command: Option<&'a str>,
    /// Location of file with author info in
    pub author_file: Option<&'a str>,
}

impl AuthorArgs for GenericArgs<'_> {
    fn author_command(&self) -> Option<&str> {
        self.author_command
    }

    fn author_file(&self) -> Option<&str> {
        self.author_file
    }
}

/// From a cli args, get the author configuration
pub trait AuthorArgs {
    /// Get the command to run to generate the authors file
    fn author_command(&self) -> Option<&str>;

    /// Get path to author file
    fn author_file(&self) -> Option<&str>;
}

/// Get authors from config
///
/// # Errors
///
/// miette error on failure of command
pub fn get_authors<'a>(args: &'a dyn AuthorArgs) -> Result<Authors<'a>> {
    let toml = args
        .author_command()
        .map_or_else(|| from_file(args), from_exec)?;
    let authors: Authors<'a> = Authors::try_from(toml)?;
    Ok(authors)
}

fn from_file(args: &dyn AuthorArgs) -> Result<String> {
    args.author_file()
        .map_or_else(|| Err(super::errors::Error::AuthorFileNotSet.into()), Ok)
        .and_then(|path| match path {
            "$HOME/.config/git-mit/mit.toml" => author_file_path(),
            _ => Ok(path.into()),
        })
        .and_then(|path| fs::read_to_string(path).into_diagnostic())
}

#[cfg(not(target_os = "windows"))]
fn author_file_path() -> Result<String> {
    let home: PathBuf = std::env::var("HOME").into_diagnostic()?.into();
    Ok(home
        .join(".config")
        .join("git-mit")
        .join("mit.toml")
        .to_string_lossy()
        .to_string())
}

#[cfg(target_os = "windows")]
fn author_file_path() -> Result<String> {
    std::env::var("APPDATA")
        .map(|x| {
            PathBuf::from(x)
                .join("git-mit")
                .join("mit.toml")
                .to_string_lossy()
                .into()
        })
        .into_diagnostic()
}

fn from_exec(command: &str) -> Result<String> {
    let commandline = shell_words::split(command).into_diagnostic()?;
    Command::new(commandline.first().unwrap_or(&String::new()))
        .stderr(Stdio::inherit())
        .args(commandline.iter().skip(1))
        .output()
        .into_diagnostic()
        .and_then(|output| {
            String::from_utf8(output.stdout).map_err(|source| {
                super::errors::Error::ExecUtf8 {
                    source,
                    command: command.to_string(),
                }
                .into()
            })
        })
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::mit::{get_authors, GenericArgs};

    #[test]
    #[cfg(unix)]
    fn unreadable_author_file_returns_error() {
        use std::os::unix::fs::PermissionsExt;

        let mut temp_file = std::env::temp_dir();
        temp_file.push(format!("unreadable_mit_test_{}.toml", std::process::id()));

        let _ = std::fs::remove_file(&temp_file);

        {
            let mut file = std::fs::File::create(&temp_file).unwrap();
            file.write_all(b"[authors]").unwrap();
        }

        let mut permissions = std::fs::metadata(&temp_file).unwrap().permissions();
        permissions.set_mode(0o000);
        std::fs::set_permissions(&temp_file, permissions).unwrap();

        let args = GenericArgs {
            author_command: None,
            author_file: Some(temp_file.to_str().unwrap()),
        };

        let result = get_authors(&args);
        assert!(
            result.is_err(),
            "expected an IO error when the author file is unreadable, but got Ok"
        );

        // Cleanup
        let mut permissions = std::fs::metadata(&temp_file).unwrap().permissions();
        permissions.set_mode(0o644);
        std::fs::set_permissions(&temp_file, permissions).unwrap();
        let _ = std::fs::remove_file(&temp_file);
    }

    use super::AuthorArgs;

    #[test]
    fn author_command_passes_through_some_value() {
        let args = GenericArgs {
            author_command: Some("echo hello"),
            author_file: None,
        };
        assert_eq!(args.author_command(), Some("echo hello"));
    }

    #[test]
    fn author_command_passes_through_none() {
        let args = GenericArgs {
            author_command: None,
            author_file: None,
        };
        assert_eq!(args.author_command(), None);
    }

    #[test]
    fn author_file_passes_through_some_value() {
        let args = GenericArgs {
            author_command: None,
            author_file: Some("/custom/path.toml"),
        };
        assert_eq!(args.author_file(), Some("/custom/path.toml"));
    }

    #[test]
    fn author_file_passes_through_none() {
        let args = GenericArgs {
            author_command: None,
            author_file: None,
        };
        assert_eq!(args.author_file(), None);
    }
}
