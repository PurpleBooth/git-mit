use std::fmt::Display;

#[derive(Debug, Eq, PartialEq)]
pub struct Problem {
    help: String,
    code: Code,
}

impl Problem {
    #[must_use]
    pub fn new(help: String, code: Code) -> Problem {
        Problem { help, code }
    }

    #[must_use]
    pub fn code(self) -> Code {
        self.code
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.help)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(i32)]
pub enum Code {
    DuplicatedTrailers = 3,
    PivotalTrackerIdMissing,
    JiraIssueKeyMissing,
    GitHubIdMissing,
    SubjectNotSeparateFromBody,
    SubjectLongerThan72Characters,
}
