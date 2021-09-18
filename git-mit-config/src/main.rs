use std::{
    env,
    path::{Path, PathBuf},
};

use cli::app::app;
use git2::{Config, Repository};
use mit_commit_message_lints::{external::Git2, lints::Lint};

use crate::errors::GitMitConfigError;

mod cli;
mod cmd;
mod errors;

fn main() -> Result<(), GitMitConfigError> {
    let lint_names: Vec<&str> = Lint::iterator()
        .map(mit_commit_message_lints::lints::Lint::name)
        .collect();
    let matches = app(&lint_names).get_matches();

    let possible: Option<Result<(), GitMitConfigError>> = [
        cmd::author_example::run_on_match,
        cmd::author_set::run_on_match,
        cmd::author_generate::run_on_match,
        cmd::lint_enable::run_on_match,
        cmd::lint_disable::run_on_match,
        cmd::lint_available::run_on_match,
        cmd::lint_enabled::run_on_match,
        cmd::lint_status::run_on_match,
        cmd::lint_generate::run_on_match,
        cmd::relates_to_template::run_on_match,
    ]
    .iter()
    .find_map(|cmd| cmd(&matches));

    if let Some(response) = possible {
        return response;
    };

    Err(GitMitConfigError::UnrecognisedLintCommand)
}

fn get_vcs(local: bool, current_dir: &Path) -> Result<Git2, GitMitConfigError> {
    let git_config = if local {
        Repository::discover(current_dir.to_path_buf())
            .and_then(|repo: Repository| repo.config())?
    } else {
        Config::open_default()?
    };

    Ok(Git2::new(git_config))
}

fn current_dir() -> Result<PathBuf, GitMitConfigError> {
    Ok(env::current_dir()?)
}
