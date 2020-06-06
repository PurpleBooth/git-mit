use regex::Regex;
use std::fmt::Display;
use std::{convert::TryFrom, fs::File, io::Read, path::PathBuf};

use crate::errors::PbCommitMessageLintsError;

#[derive(Debug, PartialEq)]
pub struct CommitMessage {
    contents: String,
    comment_char: String,
}

impl CommitMessage {
    #[must_use]
    pub fn new(contents: String) -> CommitMessage {
        let comment_char = "#".into();
        CommitMessage { contents, comment_char }
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
        let (body, trailing_comment) = self.message_parts();

        if body.is_empty() && trailing_comment.is_empty() {
            Self::new(format!("\n{}\n", trailer))
        } else if body.is_empty() {
            Self::new(format!("\n{}\n\n{}\n", trailer, trailing_comment))
        } else if trailing_comment.is_empty() {
            Self::new(format!("{}\n\n{}\n", body, trailer))
        } else {
            Self::new(format!("{}\n{}\n\n{}\n", body, trailer, trailing_comment))
        }
    }

    fn line_has_trailer(trailer: &str, line: &str) -> bool {
        line.starts_with(&format!("{}:", trailer))
    }

    fn message_parts(&self) -> (String, String) {
        let lines = self.contents.lines();

        let contents_length = lines.clone().count();
        let trailing_comment_length = lines
            .clone()
            .rev()
            .take_while(|line| line.starts_with(&self.comment_char))
            .count();
        let body_length = contents_length - trailing_comment_length;

        let body: Vec<&str> = lines.clone().take(body_length).collect();
        let trailing_comment: Vec<&str> = lines.skip(body_length).collect();

        (body.join("\n"), trailing_comment.join("\n"))
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
            CommitMessage::new("\nAnything: Some Trailer\n".into()),
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

    #[test]
    fn adding_trailer_when_message_contains_only_comments() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "

                    Trailer: Title

                    # Comments about writing a commit message
                    "
                )
                .into()
            ),
            CommitMessage::new(
                "# Comments about writing a commit message\n".into()
            ).add_trailer("Trailer: Title")
        );
    }

    #[test]
    fn adding_trailer_when_message_contains_content_with_trailing_comments() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "Message title

                    Trailer: Title

                    # Comment about committing
                    "
                )
                .into()
            ),
            CommitMessage::new(
                indoc!(
                    "Message title

                    # Comment about committing"
                )
                .into()
            ).add_trailer("Trailer: Title")
        );
    }

    #[test]
    fn adding_trailer_when_there_are_additional_comments() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "Message title

                    # Random Comment

                    Message content

                    Trailer: Title

                    # Trailing comment line 1
                    # Trailing comment line 2
                    "
                )
                .into()
            ),
            CommitMessage::new(
                indoc!(
                    "Message title

                    # Random Comment

                    Message content

                    # Trailing comment line 1
                    # Trailing comment line 2"
                )
                .into()
            ).add_trailer("Trailer: Title")
        );
    }
}
