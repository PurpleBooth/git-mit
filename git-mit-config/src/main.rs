use std::{
    env,
    path::{Path, PathBuf},
};

use cli::app::app;
use git2::{Config, Repository};
use miette::{GraphicalTheme, IntoDiagnostic, Result};
use mit_commit_message_lints::external::Git2;
use mit_lint::Lint;

use crate::errors::{
    LibGit2::{DiscoverGitRepository, ReadConfigFromGitRepository, ReadUserConfigFromGit},
    UnrecognisedLintCommand,
};

mod cli;
mod cmd;
mod errors;

fn main() -> Result<()> {
    if env::var("DEBUG_PRETTY_ERRORS").is_ok() {
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .force_graphical(true)
                    .terminal_links(false)
                    .graphical_theme(GraphicalTheme::unicode_nocolor())
                    .build(),
            )
        }))
        .unwrap();
    }

    let lint_names: Vec<&str> = Lint::all_lints().map(Lint::name).collect();
    let matches = app(&lint_names).get_matches();

    let possible: Option<Result<()>> = [
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

    Err(UnrecognisedLintCommand {}.into())
}

fn get_vcs(local: bool, current_dir: &Path) -> Result<Git2> {
    let git_config = if local {
        Repository::discover(current_dir.to_path_buf())
            .map_err(|source| DiscoverGitRepository { source })
            .and_then(|repo: Repository| {
                repo.config()
                    .map_err(|source| ReadConfigFromGitRepository { source })
            })?
    } else {
        Config::open_default().map_err(|source| ReadUserConfigFromGit { source })?
    };

    Ok(Git2::new(git_config))
}

fn current_dir() -> Result<PathBuf> {
    env::current_dir().into_diagnostic()
}
