use std::sync::LazyLock;

use mit_commit::CommitMessage;
use regex::Regex;

use crate::model::{Code, Problem, ProblemBuilder};

/// Canonical lint ID
pub const CONFIG: &str = "pivotal-tracker-id-missing";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str =
    "It's important to add the ID because it allows code to be linked back to the stories it was \
done for, it can provide a chain of custody for code for audit purposes, and it can give \
future explorers of the codebase insight into the wider organisational need behind the \
change. We may also use it for automation purposes, like generating changelogs or \
notification emails.

You can fix this by adding the Id in one of the styles below to the commit message
[Delivers #12345678]
[fixes #12345678]
[finishes #12345678]
[#12345884 #12345678]
[#12345884,#12345678]
[#12345678],[#12345884]
This will address [#12345884]";

/// Description of the problem
pub const ERROR: &str = "Your commit message is missing a Pivotal Tracker ID";

static RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\[(((finish|fix)(ed|es)?|complete[ds]?|deliver(s|ed)?) )?#\d+([, ]#\d+)*]")
        .unwrap()
});

/// Configuration for Pivotal Tracker ID linting
#[derive(Debug, Default)]
pub struct PivotalTrackerIdConfig;

/// Checks if the commit message contains a Pivotal Tracker ID
///
/// # Arguments
///
/// * `commit_message` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message does not contain a Pivotal Tracker ID
/// * `None` - If the commit message contains a Pivotal Tracker ID
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint::PivotalTrackerIdMissing;
///
/// // This should pass
/// let passing = CommitMessage::from("Subject\n\nBody\n\n[#12345678]");
/// assert!(PivotalTrackerIdMissing.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from("Subject\n\nBody");
/// assert!(PivotalTrackerIdMissing.lint(&failing).is_some());
/// ```
///
/// # Errors
///
/// This function will never return an error, only an Option<Problem>
pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit_message, &PivotalTrackerIdConfig)
}

fn lint_with_config(
    commit_message: &CommitMessage<'_>,
    _config: &PivotalTrackerIdConfig,
) -> Option<Problem> {
    Some(commit_message)
        .filter(|commit| has_problem(commit, &RE))
        .map(create_problem)
}

fn has_problem(commit_message: &CommitMessage<'_>, pattern: &Regex) -> bool {
    !commit_message.matches_pattern(pattern)
}

fn create_problem(commit_message: &CommitMessage) -> Problem {
    ProblemBuilder::new(
        ERROR,
        HELP_MESSAGE,
        Code::PivotalTrackerIdMissing,
        commit_message,
    )
    .with_label_at_last_line("No Pivotal Tracker ID")
    .with_url("https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks")
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
    fn test_commit_with_pivotal_tracker_id_passes() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[#12345678]
# Some comment
",
            None,
        );
    }

    fn test_has_missing_pivotal_tracker_id(message: &str, expected: Option<&Problem>) {
        let actual = lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    #[test]
    fn test_commit_with_multiple_pivotal_tracker_ids_passes() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[#12345678,#87654321]
# some comment
",
            None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[#12345678,#87654321,#11223344]
# some comment
",
            None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[#12345678 #87654321 #11223344]
# some comment
",
            None,
        );
    }

    #[test]
    fn test_commit_with_fix_state_change_passes() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[fix #12345678]
# some comment
",
            None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[FIX #12345678]
# some comment
",
            None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[Fix #12345678]
# some comment
",
            None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[fixed #12345678]
# some comment
",
            None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[fixes #12345678]
# some comment
",
            None,
        );
    }

    #[test]
    fn test_commit_with_complete_state_change_passes() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[complete #12345678]
# some comment
",
            None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[completed #12345678]
# some comment
",
            None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[Completed #12345678]
# some comment
",
            None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[completes #12345678]
# some comment
",
            None,
        );
    }

    #[test]
    fn test_commit_with_finish_state_change_passes() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[finish #12345678]
# some comment
",
            None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[finished #12345678]
# some comment
",
            None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[finishes #12345678]
# some comment
",
            None,
        );
    }

    #[test]
    fn test_commit_with_deliver_state_change_passes() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[deliver #12345678]
# some comment
",
            None,
        );

        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[delivered #12345678]
# some comment
",
            None,
        );
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[delivers #12345678]
# some comment
",
            None,
        );
    }

    #[test]
    fn test_commit_with_state_change_and_multiple_ids_passes() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

[fix #12345678 #12345678]
# some comment
",
            None,
        );
    }

    #[test]
    fn test_commit_with_prefixed_text_passes() {
        test_has_missing_pivotal_tracker_id(
            "An example commit

This is an example commit

Finally [fix #12345678 #12345678]
",
            None,
        );
    }

    #[test]
    fn test_commit_with_invalid_state_change_fails() {
        let message = "An example commit

This is an example commit

[fake #12345678]
# some comment
";
        test_has_missing_pivotal_tracker_id(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::PivotalTrackerIdMissing,
                &message.into(),
                Some(vec![("No Pivotal Tracker ID".to_string(), 63, 14)]),
                Some("https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks".parse().unwrap()),
            )).as_ref(),
        );
    }

    #[test]
    fn test_commit_without_pivotal_tracker_id_fails() {
        let message_1 = "An example commit

This is an example commit
";
        test_has_missing_pivotal_tracker_id(
            message_1,
            Some(&Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::PivotalTrackerIdMissing,
                &message_1.into(),
                Some(vec![("No Pivotal Tracker ID".to_string(), 19, 25)]),
                Some("https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks".parse().unwrap()),
            )),
        );

        let message_2 = "An example commit

This is an example commit

[#]
# some comment
";
        test_has_missing_pivotal_tracker_id(
            message_2,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::PivotalTrackerIdMissing,
                &message_2.into(),
                Some(vec![("No Pivotal Tracker ID".to_string(), 50, 14)]),
                Some("https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks".parse().unwrap()),
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
        let expected = "PivotalTrackerIdMissing (https://www.pivotaltracker.com/help/api?version=v5#Tracker_Updates_in_SCM_Post_Commit_Hooks)

  x Your commit message is missing a Pivotal Tracker ID
   ,-[3:1]
 2 | 
 3 | This is an example commit
   : ^^^^^^^^^^^^|^^^^^^^^^^^^
   :             `-- No Pivotal Tracker ID
   `----
  help: It's important to add the ID because it allows code to be linked back
        to the stories it was done for, it can provide a chain of custody for
        code for audit purposes, and it can give future explorers of the
        codebase insight into the wider organisational need behind the change.
        We may also use it for automation purposes, like generating changelogs
        or notification emails.
        
        You can fix this by adding the Id in one of the styles below to the
        commit message
        [Delivers #12345678]
        [fixes #12345678]
        [finishes #12345678]
        [#12345884 #12345678]
        [#12345884,#12345678]
        [#12345678],[#12345884]
        This will address [#12345884]
"        .to_string();
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

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_without_pivotal_tracker_id_fail(commit: String) -> TestResult {
        let message = CommitMessage::from(commit);
        let result = lint(&message);
        TestResult::from_bool(result.is_some())
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn test_quickcheck_commits_with_pivotal_tracker_id_pass(
        commit: String,
        commit_suffix: String,
        ids: Vec<usize>,
        prefix: Option<usize>,
    ) -> TestResult {
        if commit.starts_with('#') {
            return TestResult::discard();
        }
        if commit.ends_with("\n#") {
            return TestResult::discard();
        }
        if ids.is_empty() {
            return TestResult::discard();
        }

        let prefixes = [
            "finished ",
            "finishes ",
            "finish ",
            "fix ",
            "fixed ",
            "fixes ",
            "complete ",
            "completed ",
            "completes ",
            "delivered ",
            "delivers ",
        ];
        let prefix = prefix
            .and_then(|x| prefixes.get(x % prefixes.len()))
            .map(|x| (*x).to_string())
            .unwrap_or_default();
        let id_str: String = ids
            .iter()
            .map(|x| format!("#{x}"))
            .collect::<Vec<_>>()
            .join(",");

        let message = CommitMessage::from(format!(
            "{commit}\n[{prefix}{id_str}]\n{commit_suffix}\n# comment"
        ));
        let result = lint(&message);
        TestResult::from_bool(result.is_none())
    }
}
