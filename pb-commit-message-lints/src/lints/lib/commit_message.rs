use regex::Regex;
use std::fmt::Display;
use std::{convert::TryFrom, fs::File, io::Read, path::PathBuf};

use crate::errors::PbCommitMessageLintsError;

#[derive(Debug, PartialEq)]
pub struct CommitMessage {
    contents: String,
}

impl CommitMessage {
    #[must_use]
    pub fn new(contents: String) -> CommitMessage {
        CommitMessage { contents }
    }

    pub fn matches_pattern(&self, re: &Regex) -> bool {
        re.is_match(&self.contents)
    }

    #[must_use]
    pub fn get_trailer(&self, trailer: &str) -> Vec<&str> {
        self.contents
            .lines()
            .filter(|line: &&str| CommitMessage::line_has_trailer(trailer, line))
            .collect::<Vec<_>>()
    }

    pub fn add_trailer(&self, trailer: &str) -> Self {
        let mut message = String::from(&self.contents);

        if !message.is_empty() {
            message.push_str("\n");
        }

        message.push_str(trailer);

        message.push_str("\n");

        Self::new(message)
    }

    fn line_has_trailer(trailer: &str, line: &str) -> bool {
        line.starts_with(&format!("{}:", trailer))
    }
}

impl TryFrom<PathBuf> for CommitMessage {
    type Error = PbCommitMessageLintsError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(value)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .map_err(PbCommitMessageLintsError::from)
            .map(move |_| CommitMessage::new(buffer))
    }
}

impl Display for CommitMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

#[cfg(test)]
mod test_commit_message {
    use pretty_assertions::assert_eq;
    use regex::Regex;

    use indoc::indoc;

    use super::CommitMessage;

    #[test]
    fn with_trailers() {
        let commit = CommitMessage::new(
            indoc!(
                "Some Commit Message

                Anything: Some Trailer
                Anything: Some Trailer
                Another: Trailer
                "
            )
            .into()
        );

        assert_eq!(vec!["Another: Trailer"], commit.get_trailer("Another"));
        assert_eq!(
            vec!["Anything: Some Trailer", "Anything: Some Trailer"],
            commit.get_trailer("Anything")
        )
    }

    #[test]
    fn regex_matching() {
        let commit = CommitMessage::new(
            indoc!(
                "Some Commit Message

                Anything: Some Trailer
                Anything: Some Trailer
                Another: Trailer
                "
            )
            .into(),
        );

        assert_eq!(
            true,
            commit.matches_pattern(&Regex::new("[AB]nything:").unwrap())
        );
        assert_eq!(
            false,
            commit.matches_pattern(&Regex::new("N[oO]thing:").unwrap())
        );
    }

    #[test]
    fn adding_trailer_to_empty_message() {
        assert_eq!(
            CommitMessage::new("Anything: Some Trailer\n".into()),
            CommitMessage::new("".into()).add_trailer("Anything: Some Trailer")
        );
    }

    #[test]
    fn adding_trailer_simple_message() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "Simple commit message

                    With a description.

                    Anything: Some Trailer
                    "
                )
                .into()
            ),
            CommitMessage::new(
                indoc!(
                    "Simple commit message

                    With a description.
                    "
                )
                .into()
            ).add_trailer("Anything: Some Trailer")
        );
    }
}
