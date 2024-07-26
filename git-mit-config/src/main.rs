//! The git mit-config binary

#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

use std::{
    env,
    io::stdout,
    path::{Path, PathBuf},
};

use clap_complete::generate;
use git2::{Config, Repository};
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::{console::error_handling::miette_install, external::Git2};

use crate::{
    cli::{app, app::CliArgs},
    cmd::{author_generate, author_set},
    errors::{
        LibGit2::{DiscoverGitRepository, ReadConfigFromGitRepository, ReadUserConfigFromGit},
        UnrecognisedLintCommand,
    },
};

mod cli;
mod cmd;
mod errors;
use clap::{CommandFactory, Parser};

use crate::cli::app::Action;

fn main() -> Result<()> {
    miette_install();

    let cli_args = CliArgs::parse();

    // Simply print and exit if completion option is given.
    if let Some(completion) = cli_args.completion {
        let mut cmd = CliArgs::command();
        let name = cmd.get_name().to_string();
        generate(completion, &mut cmd, name, &mut stdout());

        std::process::exit(0);
    }

    match cli_args.action {
        Some(app::Action::Lint {
            action: app::Lint::Available { scope },
        }) => cmd::lint_available::run(scope),
        Some(app::Action::Lint {
            action: app::Lint::Enabled { scope },
        }) => cmd::lint_enabled::run(scope),
        Some(app::Action::Lint {
            action: app::Lint::Status { scope, lints },
        }) => cmd::lint_status::run(scope, lints),
        Some(app::Action::Lint {
            action: app::Lint::Enable { scope, lints },
        }) => cmd::lint_enable::run(scope, lints),
        Some(app::Action::Lint {
            action: app::Lint::Disable { scope, lints },
        }) => cmd::lint_disable::run(scope, lints),
        Some(app::Action::Lint {
            action: app::Lint::Generate { scope },
        }) => cmd::lint_generate::run(scope),
        Some(app::Action::Mit {
            action:
                app::Mit::Set {
                    scope,
                    initials,
                    name,
                    email,
                    signingkey,
                },
        }) => author_set::run(scope, &initials, name, email, signingkey),
        Some(app::Action::Mit {
            action: app::Mit::Generate { config, exec },
        }) => author_generate::run_generate(&config, &exec),
        Some(app::Action::Mit {
            action: app::Mit::Available { config, exec },
        }) => author_generate::run_available(&config, &exec),
        Some(app::Action::Mit {
            action: app::Mit::Example,
        }) => cmd::author_example::run(),
        Some(Action::Mit {
            action: app::Mit::NonCleanBehaviour { scope },
        }) => cmd::non_clean_behaviour::run(scope),
        Some(Action::Mit {
            action: app::Mit::SetNonCleanBehaviour { scope, behaviour },
        }) => cmd::non_clean_behaviour_set::run(scope, behaviour),
        Some(app::Action::RelatesTo {
            action: app::RelatesTo::Template { scope, template },
        }) => cmd::relates_to_template::run(scope, &template),
        None => Err(UnrecognisedLintCommand {}.into()),
    }
}

fn get_vcs(local: bool, current_dir: &Path) -> Result<Git2> {
    let (git_config, git_state) = if local {
        Repository::discover(current_dir)
            .map_err(|source| DiscoverGitRepository { source })
            .and_then(|repo: Repository| {
                repo.config()
                    .map_err(|source| ReadConfigFromGitRepository { source })
                    .map(|config| (config, Some(repo.state())))
            })?
    } else {
        Config::open_default()
            .map_err(|source| ReadUserConfigFromGit { source })
            .map(|config| (config, None))?
    };

    Ok(Git2::new(git_config, git_state))
}

fn current_dir() -> Result<PathBuf> {
    env::current_dir().into_diagnostic()
}
