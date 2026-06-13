use mit_commit::CommitMessage;

use crate::model::{Lints, Problem};

/// Lint a commit message
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::{lint, Lints};
/// let actual = lint(
///     &CommitMessage::from("An example commit message"),
///     Lints::available(),
/// );
/// assert!(!actual.is_empty());
/// ```
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::{Code, lint, Problem, Lints, Lint};
///
/// let message:String = "x".repeat(73).into();
/// let expected = vec![Problem::new(
///     "Your subject is longer than 72 characters".into(),
///     "It's important to keep the subject of the commit less than 72 characters because when you look at the git log, that's where it truncates the message. This means that people won't get the entirety of the information in your commit.\n\nPlease keep the subject line 72 characters or under"
///         .into(),
///     Code::SubjectLongerThan72Characters,&message.clone().into(),Some(vec![("Too long".to_string(), 72, 1)]),
///     Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
/// )];
/// let lints = Lints::new(vec![Lint::SubjectLongerThan72Characters].into_iter().collect());
/// let actual = lint(&CommitMessage::from(message), &lints);
/// assert_eq!(
///     actual, expected,
///     "Expected {:?}, found {:?}",
///     expected, actual
/// );
/// ```
#[must_use]
pub fn lint(commit_message: &CommitMessage<'_>, lints: &Lints) -> Vec<Problem> {
    lints
        .clone()
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|lint| lint.lint(commit_message))
        .collect::<Vec<Problem>>()
}
