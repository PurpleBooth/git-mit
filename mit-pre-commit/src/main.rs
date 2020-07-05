use std::convert::TryFrom;
use std::{env, process};

use console::style;

use mit_commit_message_lints::{author::vcs::get_coauthor_configuration, external::Git2};

use crate::cli::app;
use crate::errors::MitPreCommitError;

#[repr(i32)]
enum ExitCode {
    StaleAuthor = 3,
}

fn main() -> Result<(), errors::MitPreCommitError> {
    app().get_matches();

    let current_dir = env::current_dir()
        .map_err(|err| MitPreCommitError::new_io("<current_dir>".into(), &err))?;
    let mut git_config = Git2::try_from(current_dir)?;
    let co_author_configuration = get_coauthor_configuration(&mut git_config)?;

    if co_author_configuration.is_none() {
        let error = style("The details of the author of this commit are stale")
            .red()
            .bold();
        let tip = style("Can you confirm who's currently coding?\n\nIt's nice to get and give the right credit.\n\nYou can fix this by running `git mit` then the initials of whoever is coding for example:\ngit mit bt\ngit mit bt se\n").italic();
        eprintln!("{}\n\n{}", error, tip);

        process::exit(ExitCode::StaleAuthor as i32);
    }

    Ok(())
}

mod cli;

mod errors;
