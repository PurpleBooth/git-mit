use std::{env, io};

use git2::{Config, Repository};

use crate::errors::GitMitConfigError;
use crate::lints::manage_lints;
use clap_generate::generate;
use clap_generate::generators::{Bash, Elvish, Fish, Zsh};

use crate::cli::app;
use mit_commit_message_lints::lints::Lint;
use mit_commit_message_lints::{author::entities::Authors, external::vcs::Git2};
use std::convert::TryInto;

fn main() -> Result<(), GitMitConfigError> {
    let lint_names: Vec<&str> = Lint::iterator()
        .map(mit_commit_message_lints::lints::Lint::name)
        .collect();
    let matches = app(&lint_names).get_matches();

    if let Some(subcommand) = matches.subcommand_matches("completion") {
        if let Some(shell) = subcommand.value_of("shell") {
            if shell == "bash" {
                generate::<Bash, _>(
                    &mut app(&lint_names),
                    env!("CARGO_PKG_NAME"),
                    &mut io::stdout(),
                )
            } else if shell == "fish" {
                generate::<Fish, _>(
                    &mut app(&lint_names),
                    env!("CARGO_PKG_NAME"),
                    &mut io::stdout(),
                )
            } else if shell == "zsh" {
                generate::<Zsh, _>(
                    &mut app(&lint_names),
                    env!("CARGO_PKG_NAME"),
                    &mut io::stdout(),
                )
            } else if shell == "elvish" {
                generate::<Elvish, _>(
                    &mut app(&lint_names),
                    env!("CARGO_PKG_NAME"),
                    &mut io::stdout(),
                )
            }
        }

        return Ok(());
    }

    if let Some(subcommand) = matches.subcommand_matches("mit") {
        if subcommand.subcommand_matches("example").is_some() {
            let example: String = Authors::example().try_into()?;
            println!("{}", example)
        };
        Ok(())
    } else if let Some(value) = matches.subcommand_matches("lint") {
        let current_dir =
            env::current_dir().map_err(|error| GitMitConfigError::new_io("$PWD".into(), &error))?;

        let git_config = match matches.value_of("scope") {
            Some("local") => {
                Repository::discover(current_dir).and_then(|repo: Repository| repo.config())
            }
            _ => Config::open_default(),
        }?;

        let mut vcs = Git2::new(git_config);
        manage_lints(value, &mut vcs)
    } else {
        Ok(())
    }
}

mod cli;

mod lints;

mod errors;
