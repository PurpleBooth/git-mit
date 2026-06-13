use std::sync::LazyLock;

use mit_commit::CommitMessage;
use regex::Regex;

use crate::model::{Code, Problem, ProblemBuilder};

/// Canonical lint ID
pub const CONFIG: &str = "github-id-missing";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str = "It's important to add the issue ID because it allows us to link code back to the motivations for doing it, and because we can help people exploring the repository link their issues to specific bits of code.

You can fix this by adding a ID like the following examples:

#642
GH-642
AnUser/git-mit#642
AnOrganisation/git-mit#642
fixes #642

Be careful just putting '#642' on a line by itself, as '#' is the default comment character" ;
/// Description of the problem
pub const ERROR: &str = "Your commit message is missing a GitHub ID";

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)(^| )([a-zA-Z0-9_-]{3,39}/[a-zA-Z0-9-]+#|GH-|#)[0-9]+( |$)").unwrap()
});

pub struct GitHubIdConfig {
    /// Regular expression for matching GitHub IDs
    pub pattern: Regex,
}

impl Default for GitHubIdConfig {
    fn default() -> Self {
        Self {
            pattern: RE.clone(),
        }
    }
}

/// Checks if the commit message contains a GitHub ID
///
/// # Arguments
///
/// * `commit_message` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message does not contain a GitHub ID
/// * `None` - If the commit message contains a GitHub ID
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::{Lint, Lints, lint};
/// use mit_lint::Lint::GitHubIdMissing;
///
/// // This should pass
/// let passing = CommitMessage::from("Subject\n\nBody\n\nRelates-to: #123");
/// assert!(GitHubIdMissing.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from("Subject\n\nBody");
/// assert!(GitHubIdMissing.lint(&failing).is_some());
/// ```
///
/// # Errors
///
/// This function will never return an error, only an Option<Problem>
pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit_message, &GitHubIdConfig::default())
}

fn lint_with_config(
    commit_message: &CommitMessage<'_>,
    config: &GitHubIdConfig,
) -> Option<Problem> {
    Some(commit_message)
        .filter(|commit| has_problem(commit, &config.pattern))
        .map(create_problem)
}

fn has_problem(commit_message: &CommitMessage<'_>, pattern: &Regex) -> bool {
    !commit_message.matches_pattern(pattern)
}

fn create_problem(commit_message: &CommitMessage) -> Problem {
    ProblemBuilder::new(
        ERROR,
        HELP_MESSAGE,
        Code::GitHubIdMissing,
        commit_message,
    )
    .with_label_at_last_line("No GitHub ID")
    .with_url("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests")
    .build()
}

#[cfg(test)]
mod tests {

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};
    use mit_commit::CommitMessage;
    use quickcheck::TestResult;

    use super::*;
    use crate::model::{Code, Problem};

    #[test]
    fn test_github_id_with_close_keyword_passes() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

close #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

closes: #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

Closed GH-642
",
            None,
        );
    }

    #[test]
    fn test_github_id_with_fix_keyword_passes() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

fix #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

This fixes #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

fixed #642
",
            None,
        );
    }

    #[test]
    fn test_github_id_with_resolve_keyword_passes() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

fixed #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

resolve #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

resolves #642
",
            None,
        );
    }

    #[test]
    fn test_github_id_with_issue_keyword_passes() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

resolved #642
",
            None,
        );
        test_has_missing_github_id(
            "An example commit

This is an example commit

Issue #642
",
            None,
        );
    }

    #[test]
    fn test_github_id_with_gh_prefix_passes() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

GH-642
",
            None,
        );
    }

    #[test]
    fn test_github_id_with_hash_only_passes() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

#642
; Comment character is set to something else like ';'
",
            None,
        );
    }

    #[test]
    fn test_github_id_with_org_repo_format_passes() {
        test_has_missing_github_id(
            "An example commit

This is an example commit

AnUser/git-mit#642
",
            None,
        );

        test_has_missing_github_id(
            "An example commit

This is an example commit

AnOrganisation/git-mit#642
",
            None,
        );
    }

    #[test]
    fn test_commit_without_github_id_fails() {
        let message = "An example commit

This is an example commit
";
        test_has_missing_github_id(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message.into(),
                Some(vec![(String::from("No GitHub ID"), 19, 25)]),
                Some(String::from("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests")),
            )).as_ref(),
        );
    }

    #[test]
    fn test_commit_with_malformed_github_id_fails() {
        let message_1 = "An example commit

This is an example commit

H-123
";
        test_has_missing_github_id(
            message_1,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message_1.into(),
                Some(vec![("No GitHub ID".to_string(), 46, 5)]),
                Some("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests".parse().unwrap()),
            )).as_ref(),
        );
        let message_2 = "An example commit

This is an example commit

git-mit#123
";
        test_has_missing_github_id(
            message_2,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::GitHubIdMissing,
                &message_2.into(),
                Some(vec![("No GitHub ID".to_string(), 46, 11)]),
                Some("https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests".parse().unwrap()),
            )).as_ref(),
        );
    }

    #[test]
    fn test_error_report_formatting() {
        let message = "An example commit

This is an example commit
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "GitHubIdMissing (https://docs.github.com/en/github/writing-on-github/working-with-advanced-formatting/autolinked-references-and-urls#issues-and-pull-requests)

  x Your commit message is missing a GitHub ID
   ,-[3:1]
 2 | 
 3 | This is an example commit
   : ^^^^^^^^^^^^|^^^^^^^^^^^^
   :             `-- No GitHub ID
   `----
  help: It's important to add the issue ID because it allows us to link code
        back to the motivations for doing it, and because we can help people
        exploring the repository link their issues to specific bits of code.
        
        You can fix this by adding a ID like the following examples:
        
        #642
        GH-642
        AnUser/git-mit#642
        AnOrganisation/git-mit#642
        fixes #642
        
        Be careful just putting '#642' on a line by itself, as '#' is the
        default comment character
".to_string();
        assert_eq!(
            actual, expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    fn fmt_report(diag: &Report) -> String {
        let mut out = String::new();
        GraphicalReportHandler::new_themed(GraphicalTheme::none())
            .with_width(80)
            .with_links(false)
            .render_report(&mut out, diag.as_ref())
            .unwrap();
        out
    }

    fn test_has_missing_github_id(message: &str, expected: Option<&Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_without_github_id_fail(commit: String) -> TestResult {
        if commit == "\u{0}: " {
            return TestResult::discard();
        }

        let message = CommitMessage::from(commit);
        let result = lint(&message);
        TestResult::from_bool(result.is_some())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_with_gh_prefix_pass(
        commit: Option<String>,
        commit_suffix: Option<String>,
        id: usize,
    ) -> TestResult {
        if commit
            .clone()
            .filter(|x| x.starts_with('#') || x.contains("\n#"))
            .is_some()
        {
            return TestResult::discard();
        }

        let message = CommitMessage::from(format!(
            "{}GH-{}{}\n# comment",
            commit.map(|x| format!("{x} ")).unwrap_or_default(),
            id,
            commit_suffix.map(|x| format!(" {x}")).unwrap_or_default()
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_with_hash_id_pass(
        commit: Option<String>,
        commit_suffix: Option<String>,
        id: usize,
    ) -> TestResult {
        if let Some(ref initial) = commit {
            if initial.starts_with('!') || initial.contains("\n!") {
                return TestResult::discard();
            }
        }

        let message = CommitMessage::from(format!(
            "{}#{}{}\n! comment",
            commit.map(|x| format!("{x} ")).unwrap_or_default(),
            id,
            commit_suffix.map(|x| format!(" {x}")).unwrap_or_default()
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_with_org_repo_format_pass(
        commit: Option<String>,
        commit_suffix: Option<String>,
        org: String,
        repo: String,
        id: usize,
    ) -> TestResult {
        if commit.clone().filter(|x| x.starts_with('#')).is_some() {
            return TestResult::discard();
        }

        if org.is_empty()
            || org.chars().count() < 3
            || org.chars().any(|x| !x.is_ascii_alphanumeric())
        {
            return TestResult::discard();
        }
        if repo.is_empty() || repo.chars().any(|x| !x.is_ascii_alphanumeric()) {
            return TestResult::discard();
        }

        let message = CommitMessage::from(format!(
            "{}{}/{}#{}{}",
            commit.map(|x| format!("{x} ")).unwrap_or_default(),
            org,
            repo,
            id,
            commit_suffix.map(|x| format!(" {x}")).unwrap_or_default()
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }
}
