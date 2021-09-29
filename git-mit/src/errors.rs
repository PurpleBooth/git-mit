use std::{fmt::Display, string::FromUtf8Error};

use miette::{Diagnostic, LabeledSpan, SourceCode};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum GitMitError {
    #[error("failed to convert author command output to unicode")]
    #[diagnostic(
        code(git_mit::config::author::load),
        help("all characters must parse as utf8")
    )]
    ExecUtf8 {
        #[source_code]
        command: String,
        #[source]
        source: FromUtf8Error,
    },
    #[error("no mit initials provided")]
    NoAuthorInitialsProvided,
    #[error("no timeout set")]
    NoTimeoutSet,
    #[error("expected a mit file path, didn't find one")]
    AuthorFileNotSet,
}

#[derive(Error, Debug)]
#[error("could not find initial")]
pub struct UnknownAuthor {
    pub command: String,
    pub missing_initials: Vec<String>,
}

impl Diagnostic for UnknownAuthor {
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        let mut tips = vec![
            "To see a summary of your configured authors run",
            "`git mit-config mit generate`",
            "To add a new author run",
            "`git mit-config mit set eg \"Egg Sample\" egg.sample@example.com`",
        ];

        if self.missing_initials.contains(&"config".to_string()) {
            tips.push("Did you mean `git mit-config`");
        }

        if self.missing_initials.contains(&"relates-to".to_string()) {
            tips.push("Did you mean `git mit-relates-to`");
        }

        if self.missing_initials.contains(&"install".to_string()) {
            tips.push("Did you mean `git mit-install`");
        }

        let help: String = tips.join(" ");

        Some(Box::new(help))
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        Some(&self.command)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        Some(Box::new(
            self.missing_initials
                .clone()
                .into_iter()
                .filter_map(|initial| {
                    self.command
                        .find(&format!(" {} ", initial))
                        .map(|x| (x, initial))
                        .map(|(x, y)| ("Not found".to_string(), x + 1, y.len()))
                        .map(|(label, pos, offset)| LabeledSpan::new(Some(label), pos, offset))
                })
                .collect::<Vec<LabeledSpan>>()
                .into_iter(),
        ))
    }
}
