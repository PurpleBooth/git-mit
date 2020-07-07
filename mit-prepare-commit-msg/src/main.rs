use std::convert::TryFrom;
use std::path::PathBuf;
use std::{env, fs::File, io::Write};

use mit_commit::CommitMessage;
use mit_commit::Trailer;
use mit_commit_message_lints::relates::vcs::get_relate_to_configuration;
use mit_commit_message_lints::{
    external::Git2,
    mit::{get_commit_coauthor_configuration, Author},
    relates::entities::RelateTo,
};

use crate::cli::app;
use crate::errors::MitPrepareCommitMessageError;
use crate::MitPrepareCommitMessageError::MissingCommitFilePath;

mod cli;
mod errors;

fn main() -> Result<(), errors::MitPrepareCommitMessageError> {
    let matches = app().get_matches();

    let commit_message_path = matches
        .value_of("commit-message-path")
        .map(PathBuf::from)
        .ok_or_else(|| MissingCommitFilePath)?;
    let current_dir = env::current_dir()
        .map_err(|err| MitPrepareCommitMessageError::new_io("$PWD".into(), &err))?;

    let mut git_config = Git2::try_from(current_dir)?;

    if let Some(authors) = get_commit_coauthor_configuration(&mut git_config)? {
        append_coauthors_to_commit_message(commit_message_path.clone(), &authors)?
    }

    if let Some(relates_to) = get_relate_to_configuration(&mut git_config)? {
        append_relate_to_trailer_to_commit_message(commit_message_path, &relates_to)?
    }

    Ok(())
}

fn append_coauthors_to_commit_message(
    commit_message_path: PathBuf,
    authors: &[Author],
) -> Result<(), MitPrepareCommitMessageError> {
    let path = String::from(commit_message_path.to_string_lossy());
    let commit_message = CommitMessage::try_from(commit_message_path.clone())?;

    let new_message = authors
        .iter()
        .map(|x| Trailer::new("Co-authored-by", &format!("{} <{}>", x.name(), x.email())))
        .fold(commit_message, |acc, trailer| acc.add_trailer(trailer));
    File::create(commit_message_path)
        .and_then(|mut file| file.write_all(String::from(new_message).as_bytes()))
        .map_err(|err| MitPrepareCommitMessageError::new_io(path, &err))
}

fn append_relate_to_trailer_to_commit_message(
    commit_message_path: PathBuf,
    relates: &RelateTo,
) -> Result<(), MitPrepareCommitMessageError> {
    let path = String::from(commit_message_path.to_string_lossy());
    let commit_message = CommitMessage::try_from(commit_message_path.clone())?
        .add_trailer(Trailer::new("Relates-to", &relates.to()));

    File::create(commit_message_path)
        .and_then(|mut file| file.write_all(String::from(commit_message).as_bytes()))
        .map_err(|err| MitPrepareCommitMessageError::new_io(path, &err))
}
