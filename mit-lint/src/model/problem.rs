use std::fmt::Display;

use miette::{Diagnostic, LabeledSpan, SourceCode};
use mit_commit::CommitMessage;
use thiserror::Error;

use crate::model::code::Code;

/// Information about the breaking of the lint
#[derive(Error, Debug, Eq, PartialEq, Clone)]
#[error("{error}")]
pub struct Problem {
    error: String,
    tip: String,
    code: Code,
    commit_message: String,
    labels: Option<Vec<(String, usize, usize)>>,
    url: Option<String>,
}

impl Diagnostic for Problem {
    /// Unique diagnostic code that can be used to look up more information
    /// about this Diagnostic. Ideally also globally unique, and documented in
    /// the toplevel crate's documentation for easy searching. Rust path
    /// format (`foo::bar::baz`) is recommended, but more classic codes like
    /// `E0123` or Enums will work just fine.
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(format!("{:?}", self.code)))
    }

    /// Additional help text related to this Diagnostic. Do you have any
    /// advice for the poor soul who's just run into this issue?
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(&self.tip))
    }

    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.url
            .as_deref()
            .map(|x| Box::new(x) as Box<dyn Display + 'a>)
    }

    fn source_code(&self) -> Option<&dyn SourceCode> {
        if self.commit_message.is_empty() {
            None
        } else {
            Some(&self.commit_message)
        }
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        if self.commit_message.is_empty() {
            return None;
        }

        self.labels.as_ref().map(|labels| {
            Box::new(
                labels.iter().map(|(label, offset, len)| {
                    LabeledSpan::new(Some(label.clone()), *offset, *len)
                }),
            ) as Box<dyn Iterator<Item = LabeledSpan> + '_>
        })
    }
}

impl Problem {
    /// Create a new problem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::option::Option::None;
    ///
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(problem.error(), "Error title".to_string())
    /// ```
    #[must_use]
    pub fn new(
        error: String,
        tip: String,
        code: Code,
        commit_message: &CommitMessage<'_>,
        labels: Option<Vec<(String, usize, usize)>>,
        url: Option<String>,
    ) -> Self {
        Self {
            error,
            tip,
            code,
            commit_message: String::from(commit_message.clone()),
            labels,
            url,
        }
    }

    /// Get the code for this problem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::option::Option::None;
    ///
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(problem.code(), &Code::BodyWiderThan72Characters)
    /// ```
    #[must_use]
    pub const fn code(&self) -> &Code {
        &self.code
    }

    /// Get the commit message for this problem
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::option::Option::None;
    ///
    /// use mit_commit::CommitMessage;
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(
    ///     problem.commit_message(),
    ///     CommitMessage::from("Commit Message")
    /// )
    /// ```
    #[must_use]
    pub fn commit_message(&self) -> CommitMessage<'_> {
        self.commit_message.clone().into()
    }

    /// Get the descriptive title for this error
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::option::Option::None;
    ///
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(problem.error(), "Error title".to_string())
    /// ```
    #[must_use]
    pub fn error(&self) -> &str {
        &self.error
    }

    /// Get advice on how to fix the problem
    ///
    /// This should be a description of why this is a problem, and how to fix it
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::option::Option::None;
    ///
    /// use mit_lint::{Code, Problem};
    /// let problem = Problem::new(
    ///     "Error title".to_string(),
    ///     "Some advice on how to fix it".to_string(),
    ///     Code::BodyWiderThan72Characters,
    ///     &"Commit Message".into(),
    ///     None,
    ///     None,
    /// );
    ///
    /// assert_eq!(problem.tip(), "Some advice on how to fix it".to_string())
    /// ```
    #[must_use]
    pub fn tip(&self) -> &str {
        &self.tip
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::option::Option::None;

    use miette::Diagnostic;
    use mit_commit::CommitMessage;

    #[test]
    fn test_error_returns_correct_value() {
        let problem = Problem::new(
            "Some error".into(),
            String::new(),
            Code::NotConventionalCommit,
            &"".into(),
            None,
            None,
        );
        assert_eq!(problem.error(), "Some error");
    }

    #[test]
    fn test_empty_commit_returns_no_labels() {
        let problem = Problem::new(
            String::new(),
            String::new(),
            Code::NotConventionalCommit,
            &"".into(),
            Some(vec![("String".to_string(), 10_usize, 20_usize)]),
            None,
        );
        assert!(problem.labels().is_none());
    }

    #[test]
    fn test_empty_commit_returns_no_source_code() {
        let problem = Problem::new(
            String::new(),
            String::new(),
            Code::NotConventionalCommit,
            &"".into(),
            Some(vec![("String".to_string(), 10_usize, 20_usize)]),
            None,
        );
        assert!(problem.source_code().is_none());
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "Cannot be passed by value, not supported by quickcheck"
    )]
    #[quickcheck]
    fn test_error_matches_input(error: String) -> bool {
        let problem = Problem::new(
            error.clone(),
            String::new(),
            Code::NotConventionalCommit,
            &CommitMessage::from(""),
            None,
            None,
        );
        problem.error() == error
    }

    #[test]
    fn test_tip_returns_correct_value() {
        let problem = Problem::new(
            String::new(),
            "Some tip".into(),
            Code::NotConventionalCommit,
            &"".into(),
            None,
            None,
        );
        assert_eq!(problem.tip(), "Some tip");
    }

    #[allow(
        clippy::needless_pass_by_value,
        reason = "Cannot be passed by value, not supported by quickcheck"
    )]
    #[quickcheck]
    fn test_tip_matches_input(tip: String) -> bool {
        let problem = Problem::new(
            String::new(),
            tip.to_string(),
            Code::NotConventionalCommit,
            &"".into(),
            None,
            None,
        );
        problem.tip() == tip
    }

    #[test]
    fn test_code_returns_correct_value() {
        let problem = Problem::new(
            String::new(),
            String::new(),
            Code::NotConventionalCommit,
            &"".into(),
            None,
            None,
        );
        assert_eq!(problem.code(), &Code::NotConventionalCommit);
    }

    #[quickcheck]
    fn test_code_matches_input(code: Code) {
        let problem = Problem::new(String::new(), String::new(), code, &"".into(), None, None);

        assert_eq!(problem.code(), &code, "Code should match the input value");
    }

    #[test]
    fn test_commit_message_returns_correct_value() {
        let problem = Problem::new(
            String::new(),
            String::new(),
            Code::NotConventionalCommit,
            &CommitMessage::from("Commit message"),
            None,
            None,
        );
        assert_eq!(
            problem.commit_message(),
            CommitMessage::from("Commit message")
        );
    }

    #[quickcheck]
    fn test_commit_message_matches_input(message: String) {
        let problem = Problem::new(
            String::new(),
            String::new(),
            Code::NotConventionalCommit,
            &CommitMessage::from(message.clone()),
            None,
            None,
        );
        assert_eq!(
            problem.commit_message(),
            CommitMessage::from(message),
            "Commit message should match the input value"
        );
    }

    #[test]
    fn test_labels_return_correct_values() {
        let problem = Problem::new(
            String::new(),
            String::new(),
            Code::NotConventionalCommit,
            &CommitMessage::from("Commit message"),
            Some(vec![("String".to_string(), 10_usize, 20_usize)]),
            None,
        );
        assert_eq!(
            problem
                .labels()
                .unwrap()
                .map(|x| (x.label().unwrap().to_string(), x.offset(), x.len()))
                .collect::<Vec<_>>(),
            vec![("String".to_string(), 10_usize, 20_usize)]
        );
    }

    #[quickcheck]
    fn test_labels_match_input_values(start: usize, offset: usize) {
        let problem = Problem::new(
            String::new(),
            String::new(),
            Code::NotConventionalCommit,
            &CommitMessage::from("Commit message"),
            Some(vec![("String".to_string(), start, offset)]),
            None,
        );
        assert_eq!(
            problem
                .labels()
                .unwrap()
                .map(|x| (x.label().unwrap().to_string(), x.offset(), x.len()))
                .collect::<Vec<_>>(),
            vec![("String".to_string(), start, offset)]
        );
    }
}
