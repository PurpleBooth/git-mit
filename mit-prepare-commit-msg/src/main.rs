use std::{env, fs::File, io::Write};

use crate::cli::app;
use crate::errors::MitPrepareCommitMessageError;
use crate::MitPrepareCommitMessageError::MissingCommitFilePath;
use mit_commit_message_lints::{
    author::{entities::Author, vcs::get_coauthor_configuration},
    external::vcs::Git2,
    lints::lib::CommitMessage,
};
use std::convert::TryFrom;
use std::path::PathBuf;

fn main() -> Result<(), errors::MitPrepareCommitMessageError> {
    let matches = app().get_matches();

    let commit_message_path = matches
        .value_of("commit-message-path")
        .map(PathBuf::from)
        .ok_or_else(|| MissingCommitFilePath)?;
    let current_dir = env::current_dir()
        .map_err(|err| MitPrepareCommitMessageError::new_io("$PWD".into(), &err))?;

    let mut git_config = Git2::try_from(current_dir)?;

    if let Some(authors) = get_coauthor_configuration(&mut git_config)? {
        append_coauthors_to_commit_message(commit_message_path, &authors)?
    }

    Ok(())
}

mod cli;

fn append_coauthors_to_commit_message(
    commit_message_path: PathBuf,
    authors: &[Author],
) -> Result<(), MitPrepareCommitMessageError> {
    let path = String::from(commit_message_path.to_string_lossy());
    let commit_message = CommitMessage::try_from(commit_message_path.clone())?;

    let message = format!(
        "{}",
        authors
            .iter()
            .map(|x| format!("Co-authored-by: {} <{}>", x.name(), x.email()))
            .fold(commit_message, |message, trailer| message
                .add_trailer(&trailer))
    );

    File::create(commit_message_path)
        .and_then(|mut file| file.write_all(message.as_bytes()))
        .map_err(|err| MitPrepareCommitMessageError::new_io(path, &err))
}

mod errors;
