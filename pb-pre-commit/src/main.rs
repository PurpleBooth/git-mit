use std::{env, process};

use clap::{crate_authors, crate_version, App};

use pb_commit_message_lints::{
    author::vcs::get_coauthor_configuration,
    errors::PbCommitMessageLintsError,
    external::vcs::Git2,
};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter},
};

#[repr(i32)]
enum ExitCode {
    StaleAuthor = 3,
}

fn display_err_and_exit<T>(error: &PbPreCommitError) -> T {
    eprintln!("{}", error);
    process::exit(1);
}

fn main() {
    App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let current_dir = env::current_dir()
        .map_err(|err| PbPreCommitError::new_io("<current_dir>".into(), &err))
        .unwrap_or_else(|err| display_err_and_exit(&err));

    let mut git_config = Git2::try_from(current_dir)
        .map_err(PbPreCommitError::from)
        .unwrap_or_else(|err| display_err_and_exit(&err));

    let co_author_configuration = get_coauthor_configuration(&mut git_config)
        .map_err(PbPreCommitError::from)
        .unwrap_or_else(|err| display_err_and_exit(&err));

    if co_author_configuration.is_none() {
        eprintln!(
            r#"
The details of the author of this commit are a bit stale. Can you confirm who's currently coding?

It's nice to get and give the right credit.

You can fix this by running `git authors` then the initials of whoever is coding for example:
git authors bt
git authors bt se"#,
        );

        process::exit(ExitCode::StaleAuthor as i32);
    }
}

#[derive(Debug)]
enum PbPreCommitError {
    PbCommitMessageLintsError(PbCommitMessageLintsError),
    Io(String, String),
}

impl Display for PbPreCommitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PbPreCommitError::PbCommitMessageLintsError(error) => write!(f, "{}", error),
            PbPreCommitError::Io(file_source, error) => write!(
                f,
                "Failed to read from config from `{}`:\n{}",
                file_source, error
            ),
        }
    }
}

impl From<PbCommitMessageLintsError> for PbPreCommitError {
    fn from(from: PbCommitMessageLintsError) -> Self {
        PbPreCommitError::PbCommitMessageLintsError(from)
    }
}

impl Error for PbPreCommitError {}

impl PbPreCommitError {
    fn new_io(source: String, error: &std::io::Error) -> PbPreCommitError {
        PbPreCommitError::Io(source, format!("{}", error))
    }
}
