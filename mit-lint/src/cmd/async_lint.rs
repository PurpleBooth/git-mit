use futures::{StreamExt, future, stream};
use mit_commit::CommitMessage;

use crate::model::{Lints, Problem};

/// Lint a commit message
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::{async_lint, lint, Lints};
/// use tokio::runtime::Runtime;
/// let rt = Runtime::new().unwrap();
/// let actual = rt.block_on(async {
///     async_lint(
///         &CommitMessage::from("An example commit message"),
///         Lints::available(),
///     )
///     .await
/// });
/// assert!(!actual.is_empty());
/// ```
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use tokio::runtime::Runtime;
/// use mit_lint::{Problem, Code, Lints, Lint, async_lint};
///
/// let message:String = "x".repeat(73).into();
/// let expected = vec![Problem::new(
///     "Your subject is longer than 72 characters".into(),
///     "It's important to keep the subject of the commit less than 72 characters because when you look at the git log, that's where it truncates the message. This means that people won't get the entirety of the information in your commit.\n\nPlease keep the subject line 72 characters or under"
///         .into(),
///     Code::SubjectLongerThan72Characters,&message.clone().into(),
/// Some(vec![("Too long".to_string(), 72, 1)]),Some("https://git-scm.com/book/en/v2/Distributed-Git-Contributing-to-a-Project#_commit_guidelines".parse().unwrap()),
/// )];
/// let rt = Runtime::new().unwrap();
/// let actual = rt.block_on(async {
///     let lints = Lints::new(vec![Lint::SubjectLongerThan72Characters].into_iter().collect());
///     async_lint(&CommitMessage::from(message), &lints).await
/// });
/// assert_eq!(
///     actual, expected,
///     "Expected {:?}, found {:?}",
///     expected, actual
/// );
/// ```
pub async fn async_lint(commit_message: &CommitMessage<'_>, lints: &Lints) -> Vec<Problem> {
    stream::iter(lints.clone())
        .filter_map(|lint| future::ready(lint.lint(commit_message)))
        .collect::<Vec<Problem>>()
        .await
}
