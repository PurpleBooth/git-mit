use std::{env, process};

use clap::{crate_authors, crate_version, App};

use mit_commit_message_lints::{
    author::vcs::get_coauthor_configuration, errors::MitCommitMessageLintsError,
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

fn main() -> Result<(), MitPreCommitError> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let current_dir = env::current_dir()
        .map_err(|err| MitPreCommitError::new_io("<current_dir>".into(), &err))?;
    let mut git_config = Git2::try_from(current_dir)?;
    let co_author_configuration = get_coauthor_configuration(&mut git_config)?;

    if co_author_configuration.is_none() {
        eprintln!(
            "The details of the author of this commit are a bit stale. Can you confirm who's \
             currently coding?\n\nIt's nice to get and give the right credit.\n\nYou can fix this \
             by running `git mit` then the initials of whoever is coding for example:\ngit \
             mit bt\ngit mit bt se\n"
        );

        process::exit(ExitCode::StaleAuthor as i32);
    }

    Ok(())
}

#[derive(Debug)]
enum MitPreCommitError {
    PbCommitMessageLintsError(MitCommitMessageLintsError),
    Io(String, String),
}

impl Display for MitPreCommitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MitPreCommitError::PbCommitMessageLintsError(error) => write!(f, "{}", error),
            MitPreCommitError::Io(file_source, error) => write!(
                f,
                "Failed to read from config from `{}`:\n{}",
                file_source, error
            ),
        }
    }
}

impl From<MitCommitMessageLintsError> for MitPreCommitError {
    fn from(from: MitCommitMessageLintsError) -> Self {
        MitPreCommitError::PbCommitMessageLintsError(from)
    }
}

impl Error for MitPreCommitError {}

impl MitPreCommitError {
    fn new_io(source: String, error: &std::io::Error) -> MitPreCommitError {
        MitPreCommitError::Io(source, format!("{}", error))
    }
}
