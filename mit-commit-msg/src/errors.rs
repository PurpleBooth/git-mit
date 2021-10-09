use std::fmt::Display;

use miette::{Diagnostic, LabeledSpan, Result, Severity, SourceCode};
use mit_lint::Problem;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum MitCommitMsgError {
    #[error("expected file path name")]
    #[diagnostic(code(mit_commit_msg::errors::mit_commit_msg_error::commit_path_missing))]
    CommitPathMissing,
}

#[derive(Error, Debug)]
#[error("multiple lint problems")]
pub struct AggregateProblem(Vec<Problem>);

impl AggregateProblem {
    pub(crate) fn to(problems: Vec<Problem>) -> Result<()> {
        if problems.len() == 1 {
            return Err(problems.first().cloned().unwrap().into());
        } else if problems.is_empty() {
            Ok(())
        } else {
            Err(Self(problems).into())
        }
    }
}

impl Diagnostic for AggregateProblem {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        let collection = self
            .0
            .iter()
            .filter_map(|x| (x as &dyn Diagnostic).code().map(|x| x.to_string()))
            .collect::<Vec<String>>();

        if collection.is_empty() {
            None
        } else {
            Some(Box::new(collection.join(" ")))
        }
    }

    fn severity(&self) -> Option<Severity> {
        let collection = self
            .0
            .iter()
            .filter_map(miette::Diagnostic::severity)
            .collect::<Vec<Severity>>();

        if collection.is_empty() {
            None
        } else if collection.iter().any(|x| x == &Severity::Error) {
            Some(Severity::Error)
        } else if collection.iter().any(|x| x == &Severity::Warning) {
            Some(Severity::Warning)
        } else {
            Some(Severity::Advice)
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        let collection = self
            .0
            .iter()
            .filter_map(|x| (x as &dyn Diagnostic).help().map(|x| x.to_string()))
            .collect::<Vec<String>>();

        if collection.is_empty() {
            None
        } else {
            Some(Box::new(collection.join("\n\n---\n\n")))
        }
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        let collection = self
            .0
            .iter()
            .filter_map(|x| (x as &dyn Diagnostic).url().map(|x| x.to_string()))
            .collect::<Vec<String>>();

        if collection.is_empty() {
            None
        } else {
            Some(Box::new(collection.join(" ")))
        }
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        self.0.first().and_then(miette::Diagnostic::source_code)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        let collection = self
            .0
            .iter()
            .filter_map(miette::Diagnostic::labels)
            .flatten()
            .collect::<Vec<LabeledSpan>>();

        if collection.is_empty() {
            return None;
        }

        Some(Box::new(collection.into_iter()))
    }

    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        let collection = self
            .0
            .iter()
            .filter_map(miette::Diagnostic::related)
            .flatten()
            .collect::<Vec<&dyn Diagnostic>>();

        if collection.is_empty() {
            return None;
        }

        Some(Box::new(collection.into_iter()))
    }
}
