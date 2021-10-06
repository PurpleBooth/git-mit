use std::{
    convert::TryFrom,
    env,
    fs::File,
    io::Write,
    path::PathBuf,
    process::{Command, Stdio},
};
extern crate tinytemplate;
use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use mit_build_tools::completion::print_completions;
use mit_commit::{CommitMessage, Trailer};
use mit_commit_message_lints::{
    external::{Git2, Vcs},
    mit::{get_commit_coauthor_configuration, Author},
};
use serde::Serialize;
use tinytemplate::TinyTemplate;

use crate::{
    cli::app,
    errors::MitPrepareCommitMessageError,
    MitPrepareCommitMessageError::MissingCommitFilePath,
};
mod cli;
mod errors;

#[derive(Serialize)]
struct Context {
    value: String,
}
use miette::{GraphicalTheme, IntoDiagnostic, Result};
use mit_commit_message_lints::{
    mit::AuthorState,
    relates::{get_relate_to_configuration, RelateTo},
};

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
    let mut app = app();
    let matches = app.clone().get_matches();

    // Simply print and exit if completion option is given.
    if let Some(completion) = matches.value_of("completion") {
        match completion {
            "bash" => print_completions::<Bash>(&mut app),
            "elvish" => print_completions::<Elvish>(&mut app),
            "fish" => print_completions::<Fish>(&mut app),
            "powershell" => print_completions::<PowerShell>(&mut app),
            "zsh" => print_completions::<Zsh>(&mut app),
            _ => println!("Unknown completion"), // Never reached
        }

        std::process::exit(0);
    }

    let commit_message_path = match matches.value_of("commit-message-path") {
        None => Err(MissingCommitFilePath),
        Some(path) => Ok(path),
    }
    .map(PathBuf::from)?;

    let current_dir = env::current_dir().into_diagnostic()?;

    let mut git_config = Git2::try_from(current_dir)?;

    if let AuthorState::Some(authors) = get_commit_coauthor_configuration(&mut git_config)? {
        append_coauthors_to_commit_message(commit_message_path.clone(), &authors)?;
    }

    let relates_to_template = matches
        .value_of("relates-to-template")
        .map(String::from)
        .or(get_relates_to_template(&mut git_config)?);

    if let Some(exec) = matches.value_of("relates-to-exec") {
        append_relate_to_trailer_to_commit_message(
            commit_message_path,
            &get_relates_to_from_exec(exec)?,
            relates_to_template,
        )?;
    } else if let Some(relates_to) = get_relate_to_configuration(&mut git_config)? {
        append_relate_to_trailer_to_commit_message(
            commit_message_path,
            &relates_to,
            relates_to_template,
        )?;
    }

    Ok(())
}

fn get_relates_to_template(vcs: &mut Git2) -> Result<Option<String>> {
    Ok(vcs.get_str("mit.relate.template")?.map(String::from))
}

fn append_coauthors_to_commit_message(
    commit_message_path: PathBuf,
    authors: &[Author],
) -> Result<()> {
    let _path = String::from(commit_message_path.to_string_lossy());
    let mut commit_message = CommitMessage::try_from(commit_message_path.clone())?;

    let trailers = authors
        .iter()
        .map(|x| Trailer::new("Co-authored-by", &format!("{} <{}>", x.name(), x.email())))
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
    relates: &RelateTo,
    template: Option<String>,
) -> Result<()> {
    let _path = String::from(commit_message_path.to_string_lossy());
    let commit_message = CommitMessage::try_from(commit_message_path.clone())?;

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
    let trailer = Trailer::new("Relates-to", &value);
    add_trailer_if_not_existing(commit_message_path, &commit_message, &trailer)?;

    Ok(())
}

fn add_trailer_if_not_existing(
    commit_message_path: PathBuf,
    commit_message: &CommitMessage,
    trailer: &Trailer,
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

fn get_relates_to_from_exec(command: &str) -> Result<RelateTo> {
    let commandline = shell_words::split(command).into_diagnostic()?;
    Command::new(commandline.first().unwrap_or(&String::from("")))
        .stderr(Stdio::inherit())
        .args(commandline.iter().skip(1).collect::<Vec<_>>())
        .output()
        .into_diagnostic()
        .and_then(|x| {
            Ok(RelateTo::new(
                &String::from_utf8(x.stdout).into_diagnostic()?,
            ))
        })
}
