use std::fmt::Display;

use miette::{Diagnostic, LabeledSpan, SourceCode};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum GitMitError {
    #[error("no mit initials provided")]
    #[diagnostic(
        url(docsrs),
        code(git_mit::errors::git_mit_error::no_author_initials_provided)
    )]
    NoAuthorInitialsProvided,
    #[error("no timeout set")]
    #[diagnostic(url(docsrs), code(git_mit::errors::git_mit_error::no_timeout_set))]
    NoTimeoutSet,
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
                .flat_map(|initial| {
                    let mut matches = self
                        .command
                        .match_indices(&format!(" {} ", initial))
                        .map(|x| (x, initial.clone()))
                        .map(|(x, y)| ("Not found".to_string(), x.0 + 1, y.len()))
                        .map(|(label, pos, offset)| LabeledSpan::new(Some(label), pos, offset))
                        .collect::<Vec<_>>();

                    if self.command.ends_with(&initial) {
                        matches.push(LabeledSpan::new(
                            Some("Not found".to_string()),
                            self.command.len() - initial.len(),
                            initial.len(),
                        ));
                    }

                    matches
                })
                .collect::<Vec<LabeledSpan>>()
                .into_iter(),
        ))
    }
}
