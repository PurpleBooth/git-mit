use std::convert::TryInto;

use mit_commit::CommitMessage;
use thiserror::Error;

use crate::lints::checks;
use crate::lints::lib::Lints;
use crate::lints::lib::Problem;

/// The lints that are supported
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash, Ord, PartialOrd)]
pub enum Lint {
    DuplicatedTrailers,
    PivotalTrackerIdMissing,
    JiraIssueKeyMissing,
    GitHubIdMissing,
    SubjectNotSeparateFromBody,
    SubjectLongerThan72Characters,
    SubjectNotCapitalized,
    SubjectEndsWithPeriod,
    BodyWiderThan72Characters,
    NotConventionalCommit,
    NotEmojiLog,
}

pub(crate) const CONFIG_KEY_PREFIX: &str = "mit.lint";

impl std::convert::TryFrom<&str> for Lint {
    type Error = Error;

    fn try_from(from: &str) -> Result<Self, Self::Error> {
        Lint::iterator()
            .zip(Lint::iterator().map(|lint| format!("{}", lint)))
            .filter_map(|(lint, name): (Lint, String)| if name == from { Some(lint) } else { None })
            .collect::<Vec<Lint>>()
            .first()
            .copied()
            .ok_or_else(|| Error::LintNotFound(from.into()))
    }
}

impl std::convert::From<Lint> for String {
    fn from(from: Lint) -> Self {
        format!("{}", from)
    }
}

impl From<Lint> for &str {
    fn from(lint: Lint) -> Self {
        lint.name()
    }
}

impl Lint {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Lint::DuplicatedTrailers => checks::duplicate_trailers::CONFIG,
            Lint::PivotalTrackerIdMissing => checks::missing_pivotal_tracker_id::CONFIG,
            Lint::JiraIssueKeyMissing => checks::missing_jira_issue_key::CONFIG,
            Lint::GitHubIdMissing => checks::missing_github_id::CONFIG,
            Lint::SubjectNotSeparateFromBody => checks::subject_not_seperate_from_body::CONFIG,
            Lint::SubjectLongerThan72Characters => {
                checks::subject_longer_than_72_characters::CONFIG
            }
            Lint::SubjectNotCapitalized => checks::subject_not_capitalized::CONFIG,
            Lint::SubjectEndsWithPeriod => checks::subject_line_ends_with_period::CONFIG,
            Lint::BodyWiderThan72Characters => checks::body_wider_than_72_characters::CONFIG,
            Lint::NotConventionalCommit => checks::not_conventional_commit::CONFIG,
            Lint::NotEmojiLog => checks::not_emoji_log::CONFIG,
        }
    }
}

lazy_static! {
    static ref ALL_LINTS: [Lint; 11] = [
        Lint::DuplicatedTrailers,
        Lint::PivotalTrackerIdMissing,
        Lint::JiraIssueKeyMissing,
        Lint::SubjectNotSeparateFromBody,
        Lint::GitHubIdMissing,
        Lint::SubjectLongerThan72Characters,
        Lint::SubjectNotCapitalized,
        Lint::SubjectEndsWithPeriod,
        Lint::BodyWiderThan72Characters,
        Lint::NotConventionalCommit,
        Lint::NotEmojiLog,
    ];
    static ref DEFAULT_ENABLED_LINTS: [Lint; 4] = [
        Lint::DuplicatedTrailers,
        Lint::SubjectNotSeparateFromBody,
        Lint::SubjectLongerThan72Characters,
        Lint::BodyWiderThan72Characters,
    ];
}

impl Lint {
    pub fn iterator() -> impl Iterator<Item = Lint> {
        ALL_LINTS.iter().copied()
    }

    #[must_use]
    pub fn enabled_by_default(self) -> bool {
        DEFAULT_ENABLED_LINTS.contains(&self)
    }

    #[must_use]
    pub fn config_key(self) -> String {
        format!("{}.{}", CONFIG_KEY_PREFIX, self)
    }

    #[must_use]
    pub fn lint(self, commit_message: &CommitMessage) -> Option<Problem> {
        match self {
            Lint::DuplicatedTrailers => checks::duplicate_trailers::lint(commit_message),
            Lint::PivotalTrackerIdMissing => {
                checks::missing_pivotal_tracker_id::lint(commit_message)
            }
            Lint::JiraIssueKeyMissing => checks::missing_jira_issue_key::lint(commit_message),
            Lint::GitHubIdMissing => checks::missing_github_id::lint(commit_message),
            Lint::SubjectNotSeparateFromBody => {
                checks::subject_not_seperate_from_body::lint(commit_message)
            }
            Lint::SubjectLongerThan72Characters => {
                checks::subject_longer_than_72_characters::lint(commit_message)
            }
            Lint::SubjectNotCapitalized => checks::subject_not_capitalized::lint(commit_message),
            Lint::SubjectEndsWithPeriod => {
                checks::subject_line_ends_with_period::lint(commit_message)
            }
            Lint::BodyWiderThan72Characters => {
                checks::body_wider_than_72_characters::lint(commit_message)
            }
            Lint::NotConventionalCommit => checks::not_conventional_commit::lint(commit_message),
            Lint::NotEmojiLog => checks::not_emoji_log::lint(commit_message),
        }
    }

    /// Try and convert a list of names into lints
    ///
    /// # Errors
    /// If the lint does not exist
    pub fn from_names(names: Vec<&str>) -> Result<Vec<Lint>, super::lints::Error> {
        let lints: Lints = names.try_into()?;
        Ok(lints.into_iter().collect())
    }
}

#[cfg(test)]
mod tests_lints {
    use std::convert::TryInto;

    use pretty_assertions::assert_eq;

    use crate::lints::Lint;

    #[test]
    fn it_is_convertible_to_string() {
        let string: String = Lint::PivotalTrackerIdMissing.into();
        assert_eq!("pivotal-tracker-id-missing".to_string(), string)
    }

    #[test]
    fn it_can_be_created_from_string() {
        let lint: Lint = "pivotal-tracker-id-missing".try_into().unwrap();
        assert_eq!(Lint::PivotalTrackerIdMissing, lint)
    }

    #[test]
    fn it_is_printable() {
        assert_eq!(
            "pivotal-tracker-id-missing",
            &format!("{}", Lint::PivotalTrackerIdMissing)
        )
    }

    #[test]
    fn i_can_get_all_the_lints() {
        let all: Vec<Lint> = Lint::iterator().collect();
        assert_eq!(
            all,
            vec![
                Lint::DuplicatedTrailers,
                Lint::PivotalTrackerIdMissing,
                Lint::JiraIssueKeyMissing,
                Lint::SubjectNotSeparateFromBody,
                Lint::GitHubIdMissing,
                Lint::SubjectLongerThan72Characters,
                Lint::SubjectNotCapitalized,
                Lint::SubjectEndsWithPeriod,
                Lint::BodyWiderThan72Characters,
                Lint::NotConventionalCommit,
                Lint::NotEmojiLog,
            ]
        )
    }

    #[test]
    fn i_can_get_if_a_lint_is_enabled_by_default() {
        assert_eq!(Lint::DuplicatedTrailers.enabled_by_default(), true);
        assert_eq!(Lint::PivotalTrackerIdMissing.enabled_by_default(), false);
        assert_eq!(Lint::JiraIssueKeyMissing.enabled_by_default(), false);
        assert_eq!(Lint::SubjectNotSeparateFromBody.enabled_by_default(), true);
        assert_eq!(Lint::GitHubIdMissing.enabled_by_default(), false);
    }
}

impl std::fmt::Display for Lint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Lint not found: {0}")]
    LintNotFound(String),
}
