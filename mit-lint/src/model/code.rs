use quickcheck::{Arbitrary, Gen};
use strum_macros::EnumIter;

/// Error codes for lints that have failed
///
/// Useful for exit codes and other user facing things
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
#[repr(i32)]
pub enum Code {
    /// Unique ID for `InitialNotMatchedToAuthor` failure
    InitialNotMatchedToAuthor = 3,
    /// Unique ID for `UnparsableAuthorFile` failure
    UnparsableAuthorFile,
    /// Unique ID for `StaleAuthor` failure
    StaleAuthor,
    /// Unique ID for `DuplicatedTrailers` failure
    DuplicatedTrailers,
    /// Unique ID for `PivotalTrackerIdMissing` failure
    PivotalTrackerIdMissing,
    /// Unique ID for `JiraIssueKeyMissing` failure
    JiraIssueKeyMissing,
    /// Unique ID for `GitHubIdMissing` failure
    GitHubIdMissing,
    /// Unique ID for `SubjectNotSeparateFromBody` failure
    SubjectNotSeparateFromBody,
    /// Unique ID for `SubjectLongerThan72Characters` failure
    SubjectLongerThan72Characters,
    /// Unique ID for `SubjectNotCapitalized` failure
    SubjectNotCapitalized,
    /// Unique ID for `SubjectEndsWithPeriod` failure
    SubjectEndsWithPeriod,
    /// Unique ID for `BodyWiderThan72Characters` failure
    BodyWiderThan72Characters,
    /// Unique ID for `NotConventionalCommit` failure
    NotConventionalCommit,
    /// Unique ID for `NotEmojiLog` failure
    NotEmojiLog,
}

impl Arbitrary for Code {
    fn arbitrary(g: &mut Gen) -> Self {
        *g.choose(&Self::get_codes()).unwrap()
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        quickcheck::empty_shrinker()
    }
}

impl Code {
    const fn get_codes() -> [Self; 14] {
        [
            Self::InitialNotMatchedToAuthor,
            Self::UnparsableAuthorFile,
            Self::StaleAuthor,
            Self::DuplicatedTrailers,
            Self::PivotalTrackerIdMissing,
            Self::JiraIssueKeyMissing,
            Self::GitHubIdMissing,
            Self::SubjectNotSeparateFromBody,
            Self::SubjectLongerThan72Characters,
            Self::SubjectNotCapitalized,
            Self::SubjectEndsWithPeriod,
            Self::BodyWiderThan72Characters,
            Self::NotConventionalCommit,
            Self::NotEmojiLog,
        ]
    }
}
