use std::{convert::TryFrom, env};

use mit_commit_message_lints::{external::Git2, mit::get_commit_coauthor_configuration};

use crate::{cli::app, errors::MitPreCommitError};

fn main() -> Result<(), errors::MitPreCommitError> {
    app().get_matches();

    let current_dir = env::current_dir()
        .map_err(|err| MitPreCommitError::new_io("<current_dir>".into(), &err))?;
    let mut git_config = Git2::try_from(current_dir)?;
    let co_author_configuration = get_commit_coauthor_configuration(&mut git_config)?;

    if co_author_configuration.is_none() {
        mit_commit_message_lints::console::exit_stale_author();
    }

    Ok(())
}

mod cli;

mod errors;
