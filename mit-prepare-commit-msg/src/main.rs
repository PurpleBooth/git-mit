//! prepare-commit-msg binary
#![warn(clippy::nursery)]
#![deny(
    unused,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::cargo,
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::pedantic,
    non_fmt_panics
)]
#![allow(clippy::multiple_crate_versions)]
use std::{
    convert::TryFrom,
    env,
    fs::File,
    io::{stdout, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use miette::{IntoDiagnostic, Result};
use mit_commit::{CommitMessage, Trailer};
use mit_commit_message_lints::{
    console::error_handling::miette_install,
    external::{Git2, RepoState, Vcs},
    mit::{
        cmd::get_config_non_clean_behaviour::get_config_non_clean_behaviour,
        get_commit_coauthor_configuration,
        lib::non_clean_behaviour::BehaviourOption,
        Author,
        AuthorState,
    },
    relates::{get_relate_to_configuration, RelateTo},
};
use serde::Serialize;
use tinytemplate::TinyTemplate;

use crate::{cli::Args, errors::MitPrepareCommitMessageError::MissingCommitFilePath};

mod cli;
mod errors;

#[derive(Serialize)]
struct Context<'a> {
    value: &'a str,
}

fn main() -> Result<()> {
    miette_install();

    let cli_args = Args::parse();

    // Simply print and exit if completion option is given.
    if let Some(completion) = cli_args.completion {
        let mut cmd = Args::command();
        let name = cmd.get_name().to_string();
        generate(completion, &mut cmd, name, &mut stdout());

        return Ok(());
    }

    let commit_message_path = cli_args.commit_message_path.ok_or(MissingCommitFilePath)?;

    let current_dir = env::current_dir().into_diagnostic()?;

    let git_config = Git2::try_from(current_dir)?;

    if matches!(
        (
            cli_args
                .non_clean_behaviour_option
                .unwrap_or(get_config_non_clean_behaviour(&git_config)?),
            git_config.state()
        ),
        (
            BehaviourOption::NoChange,
            Some(
                RepoState::Merge
                    | RepoState::Revert
                    | RepoState::RevertSequence
                    | RepoState::CherryPick
                    | RepoState::CherryPickSequence
                    | RepoState::Bisect
                    | RepoState::Rebase
                    | RepoState::RebaseInteractive
                    | RepoState::RebaseMerge
                    | RepoState::ApplyMailbox
                    | RepoState::ApplyMailboxOrRebase
            )
        )
    ) {
        return Ok(());
    }

    if let AuthorState::Some(authors) = get_commit_coauthor_configuration(&git_config)? {
        append_coauthors_to_commit_message(commit_message_path.clone(), &authors)?;
    }

    let relates_to_template = cli_args
        .relates_to_template
        .or(get_relates_to_template(&git_config)?);

    if let Some(exec) = cli_args.relates_to_exec {
        append_relate_to_trailer_to_commit_message(
            commit_message_path,
            &get_relates_to_from_exec(&exec)?,
            relates_to_template,
        )?;
    } else if let Some(relates_to) = get_relate_to_configuration(&git_config)? {
        append_relate_to_trailer_to_commit_message(
            commit_message_path,
            &relates_to,
            relates_to_template,
        )?;
    }

    Ok(())
}

fn get_relates_to_template(vcs: &Git2) -> Result<Option<String>> {
    Ok(vcs.get_str("mit.relate.template")?.map(String::from))
}

fn append_coauthors_to_commit_message(
    commit_message_path: PathBuf,
    authors: &[Author<'_>],
) -> Result<()> {
    let _path = String::from(commit_message_path.to_string_lossy());
    let mut commit_message =
        CommitMessage::try_from(commit_message_path.clone()).into_diagnostic()?;

    let trailers = authors
        .iter()
        .map(|x| {
            Trailer::new(
                "Co-authored-by".into(),
                format!("{} <{}>", x.name(), x.email()).into(),
            )
        })
        .collect::<Vec<_>>();

    for trailer in trailers {
        if !commit_message
            .get_trailers()
            .iter()
            .any(|existing_trailer| &trailer == existing_trailer)
        {
            commit_message = commit_message.add_trailer(trailer);
        }
    }

    File::create(commit_message_path)
        .and_then(|mut file| file.write_all(String::from(commit_message).as_bytes()))
        .into_diagnostic()
}

fn append_relate_to_trailer_to_commit_message(
    commit_message_path: PathBuf,
    relates: &RelateTo<'_>,
    template: Option<String>,
) -> Result<()> {
    let _path = String::from(commit_message_path.to_string_lossy());
    let commit_message = CommitMessage::try_from(commit_message_path.clone()).into_diagnostic()?;

    let mut tt = TinyTemplate::new();
    let defaulted_template = template.unwrap_or_else(|| "{ value }".to_string());
    tt.add_template("template", &defaulted_template)
        .into_diagnostic()?;
    let value = tt
        .render(
            "template",
            &Context {
                value: relates.to(),
            },
        )
        .into_diagnostic()?;
    let trailer = Trailer::new("Relates-to".into(), value.into());
    add_trailer_if_not_existing(commit_message_path, &commit_message, &trailer)?;

    Ok(())
}

fn add_trailer_if_not_existing(
    commit_message_path: PathBuf,
    commit_message: &CommitMessage<'_>,
    trailer: &Trailer<'_>,
) -> Result<()> {
    if commit_message
        .get_trailers()
        .iter()
        .any(|existing_trailer| trailer == existing_trailer)
    {
        Ok(())
    } else {
        File::create(commit_message_path)
            .and_then(|mut file| {
                file.write_all(String::from(commit_message.add_trailer(trailer.clone())).as_bytes())
            })
            .into_diagnostic()
    }
}

fn get_relates_to_from_exec(command: &str) -> Result<RelateTo<'_>> {
    let commandline = shell_words::split(command).into_diagnostic()?;
    Command::new(commandline.first().unwrap_or(&String::new()))
        .stderr(Stdio::inherit())
        .args(commandline.iter().skip(1))
        .output()
        .into_diagnostic()
        .and_then(|x| {
            Ok(RelateTo::from(
                String::from_utf8(x.stdout).into_diagnostic()?,
            ))
        })
}
