use std::convert::TryFrom;
use std::path::PathBuf;
use std::{env, fs::File, io, io::Write};

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
use std::process::{Command, Stdio};

mod cli;
mod errors;

const PROBABLY_SAFE_FALLBACK_SHELL: &str = "/bin/sh";

fn main() -> Result<(), errors::MitPrepareCommitMessageError> {
    let matches = app().get_matches();

    let commit_message_path = matches
        .value_of("commit-message-path")
        .map(PathBuf::from)
        .ok_or(MissingCommitFilePath)?;
    let current_dir = env::current_dir()
        .map_err(|err| MitPrepareCommitMessageError::new_io("$PWD".into(), &err))?;

    let mut git_config = Git2::try_from(current_dir)?;

    if let Some(authors) = get_commit_coauthor_configuration(&mut git_config)? {
        append_coauthors_to_commit_message(commit_message_path.clone(), &authors)?
    }

    if let Some(exec) = matches.value_of("relates-to-exec") {
        append_relate_to_trailer_to_commit_message(
            commit_message_path,
            &get_relates_to_from_exec(exec)?,
        )?
    } else if let Some(relates_to) = get_relate_to_configuration(&mut git_config)? {
        append_relate_to_trailer_to_commit_message(commit_message_path, &relates_to)?
    }

    Ok(())
}

fn append_coauthors_to_commit_message(
    commit_message_path: PathBuf,
    authors: &[Author],
) -> Result<(), MitPrepareCommitMessageError> {
    let path = String::from(commit_message_path.to_string_lossy());
    let mut commit_message = CommitMessage::try_from(commit_message_path.clone())?;

    let trailers = authors
        .iter()
        .map(|x| Trailer::new("Co-authored-by", &format!("{} <{}>", x.name(), x.email())))
        .collect::<Vec<_>>();

    for trailer in trailers {
        if commit_message
            .get_trailers()
            .iter()
            .find(|existing_trailer| &&trailer == existing_trailer)
            .is_none()
        {
            commit_message = commit_message.add_trailer(trailer);
        }
    }

    File::create(commit_message_path)
        .and_then(|mut file| file.write_all(String::from(commit_message).as_bytes()))
        .map_err(|err| MitPrepareCommitMessageError::new_io(path, &err))
}

fn append_relate_to_trailer_to_commit_message(
    commit_message_path: PathBuf,
    relates: &RelateTo,
) -> Result<(), MitPrepareCommitMessageError> {
    let path = String::from(commit_message_path.to_string_lossy());
    let commit_message = CommitMessage::try_from(commit_message_path.clone())?;

    let trailer = Trailer::new("Relates-to", &relates.to());
    add_trailer_if_not_existing(commit_message_path, &commit_message, &trailer)
        .map_err(|err| MitPrepareCommitMessageError::new_io(path, &err))?;

    Ok(())
}

fn add_trailer_if_not_existing(
    commit_message_path: PathBuf,
    commit_message: &CommitMessage,
    trailer: &Trailer,
) -> Result<(), io::Error> {
    if commit_message
        .get_trailers()
        .iter()
        .find(|existing_trailer| &trailer == existing_trailer)
        .is_none()
    {
        File::create(commit_message_path).and_then(|mut file| {
            file.write_all(String::from(commit_message.add_trailer(trailer.clone())).as_bytes())
        })
    } else {
        Ok(())
    }
}

fn get_relates_to_from_exec(command: &str) -> Result<RelateTo, MitPrepareCommitMessageError> {
    let shell = env::var("SHELL").unwrap_or_else(|_| PROBABLY_SAFE_FALLBACK_SHELL.into());
    Command::new(shell)
        .stderr(Stdio::inherit())
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|error| MitPrepareCommitMessageError::new_exec(command.into(), &error))
        .and_then(|x| {
            Ok(RelateTo::new(
                &String::from_utf8(x.stdout).map_err(MitPrepareCommitMessageError::from)?,
            ))
        })
}
