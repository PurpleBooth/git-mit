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
    io::{Write, stdout},
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use miette::{IntoDiagnostic, Result};
use mit_commit::{CommitMessage, Trailer};
use mit_commit_message_lints::{
    console::error_handling::miette_install,
    external::{self, Git2, RepoState, Vcs},
    mit::{
        Author, AuthorState,
        cmd::{
            get_config_non_clean_behaviour::get_config_non_clean_behaviour,
            get_config_rotation::get_config_rotation, rotate_authors::rotate_authors,
        },
        get_commit_coauthor_configuration,
        lib::non_clean_behaviour::BehaviourOption,
    },
    relates::{RelateTo, get_relate_to_configuration},
};

use crate::{cli::Args, errors::MitPrepareCommitMessageError};

mod cli;
mod errors;

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

    let current_dir = env::current_dir().into_diagnostic()?;
    let commit_message_path =
        external::resolve_commit_message_path(cli_args.commit_message_path, &current_dir)?;

    let git_config = Git2::try_from(current_dir.clone())?;

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

        // Rotate primary author for the next commit if rotation is enabled
        let rotation = get_config_rotation(&git_config)?;
        if let Some(strategy) = rotation {
            let mut mutable_config = Git2::try_from(current_dir)?;
            rotate_authors(&mut mutable_config, strategy)?;
        }
    }

    let relates_to_template = cli_args
        .relates_to_template
        .or(get_relates_to_template(&git_config)?);

    if let Some(exec) = cli_args.relates_to_exec {
        append_relate_to_trailer_to_commit_message(
            commit_message_path,
            &get_relates_to_from_exec(&exec)?,
            relates_to_template.as_deref(),
        )?;
    } else if let Some(relates_to) = get_relate_to_configuration(&git_config)? {
        append_relate_to_trailer_to_commit_message(
            commit_message_path,
            &relates_to,
            relates_to_template.as_deref(),
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
    template: Option<&str>,
) -> Result<()> {
    let _path = String::from(commit_message_path.to_string_lossy());
    let commit_message = CommitMessage::try_from(commit_message_path.clone()).into_diagnostic()?;

    let defaulted_template = template.unwrap_or("{ value }");
    let value = render_relates_to_template(defaulted_template, relates.to())?;
    let trailer = Trailer::new("Relates-to".into(), value.into());
    add_trailer_if_not_existing(commit_message_path, &commit_message, &trailer)?;

    Ok(())
}

/// Substitute the `{ value }` placeholder in a relates-to template.
///
/// The inner whitespace is optional, so `{value}` works too. Everything outside
/// a placeholder is literal text; any other name inside braces, or a `{` with
/// no matching `}`, is an error.
fn render_relates_to_template(template: &str, value: &str) -> Result<String> {
    let invalid = |span_offset: usize, span_len: usize| {
        MitPrepareCommitMessageError::InvalidRelatesToTemplate {
            src: template.to_string(),
            span: (span_offset, span_len).into(),
        }
        .into()
    };

    let mut rendered = String::with_capacity(template.len());
    let mut remainder = template;
    while let Some(open) = remainder.find('{') {
        rendered.push_str(&remainder[..open]);
        let after_open = &remainder[open + 1..];
        let Some(close) = after_open.find('}') else {
            return Err(invalid(open, 1));
        };
        if after_open[..close].trim() != "value" {
            return Err(invalid(open, close + 2));
        }
        rendered.push_str(value);
        remainder = &after_open[close + 1..];
    }
    rendered.push_str(remainder);

    Ok(rendered)
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
    let output = Command::new(commandline.first().unwrap_or(&String::new()))
        .stderr(Stdio::inherit())
        .args(commandline.iter().skip(1))
        .output()
        .into_diagnostic()?;

    if !output.status.success() {
        return Err(MitPrepareCommitMessageError::RelatesToExecFailed {
            exit_code: output.status.code().unwrap_or(-1),
        }
        .into());
    }

    Ok(RelateTo::from(
        String::from_utf8(output.stdout)
            .into_diagnostic()?
            .trim()
            .to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_relates_to_from_exec_trims_trailing_newline() {
        let result = get_relates_to_from_exec("echo '[#123]'").unwrap();
        assert_eq!(
            result,
            RelateTo::from("[#123]"),
            "Expected the relates-to value to be trimmed of trailing newline, got {result:?}"
        );
    }

    #[test]
    fn test_get_relates_to_from_exec_fails_on_nonzero_exit() {
        let result = get_relates_to_from_exec("false");
        assert!(
            result.is_err(),
            "Expected an error when the exec command exits non-zero"
        );
    }

    #[test]
    fn renders_the_default_template() {
        let result = render_relates_to_template("{ value }", "[#123]").unwrap();
        assert_eq!(
            result, "[#123]",
            "the default template must substitute the value, got {result:?}"
        );
    }

    #[test]
    fn renders_a_custom_template_with_surrounding_text() {
        let result = render_relates_to_template("Relates to { value }, see also", "[#1]").unwrap();
        assert_eq!(
            result, "Relates to [#1], see also",
            "text around the placeholder must be kept verbatim, got {result:?}"
        );
    }

    #[test]
    fn accepts_the_placeholder_without_inner_whitespace() {
        let result = render_relates_to_template("{value}", "abc").unwrap();
        assert_eq!(
            result, "abc",
            "`{{value}}` without spaces must also substitute, got {result:?}"
        );
    }

    #[test]
    fn substitutes_every_placeholder() {
        let result = render_relates_to_template("{ value } and { value }", "x").unwrap();
        assert_eq!(
            result, "x and x",
            "every occurrence must be substituted, got {result:?}"
        );
    }

    #[test]
    fn passes_braces_in_the_value_through() {
        let result = render_relates_to_template("{ value }", "{ not a placeholder }").unwrap();
        assert_eq!(
            result, "{ not a placeholder }",
            "the substituted value must not be re-scanned for placeholders, got {result:?}"
        );
    }

    #[test]
    fn renders_an_empty_template_to_an_empty_value() {
        let result = render_relates_to_template("", "abc").unwrap();
        assert_eq!(
            result, "",
            "an empty template must render to an empty string, got {result:?}"
        );
    }

    #[test]
    fn errors_on_an_unknown_placeholder() {
        let result = render_relates_to_template("no { value } but { typo }", "abc");
        assert!(
            result.is_err(),
            "an unknown placeholder name must be an error, got {:?}",
            result.map(|_| ()).map_err(|err| err.to_string())
        );
    }

    #[test]
    fn errors_on_an_unterminated_placeholder() {
        let result = render_relates_to_template("value is { value", "abc");
        assert!(
            result.is_err(),
            "an opening brace with no closing brace must be an error, got {:?}",
            result.map(|_| ()).map_err(|err| err.to_string())
        );
    }

    #[test]
    fn errors_on_an_empty_placeholder() {
        let result = render_relates_to_template("value is {}", "abc");
        assert!(
            result.is_err(),
            "an empty placeholder must be an error, got {:?}",
            result.map(|_| ()).map_err(|err| err.to_string())
        );
    }
}
