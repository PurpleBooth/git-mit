use regex::{Regex, RegexBuilder};
use std::{
    convert::TryFrom,
    fmt::Display,
    fs::File,
    io,
    io::Read,
    path::PathBuf,
    str::{FromStr, Lines},
};
use thiserror::Error;

use super::Trailer;

const SCISSORS_LINE: &str = "------------------------ >8 ------------------------";

#[derive(Debug, Clone, PartialEq)]
pub struct CommitMessage {
    contents: String,
    comment_char: String,
}

impl CommitMessage {
    #[must_use]
    pub fn new(contents: String) -> CommitMessage {
        let comment_char = detect_comment_char(&contents).into();
        CommitMessage {
            contents,
            comment_char,
        }
    }

    pub fn matches_pattern(&self, re: &Regex) -> bool {
        re.is_match(&self.contents)
    }

    fn content_lines(&self) -> Vec<String> {
        let scissors = self.scissors_line();
        self.contents
            .lines()
            .take_while(|line| scissors != *line)
            .filter_map(|line| {
                if line.starts_with(&self.comment_char) {
                    None
                } else {
                    Some(line.into())
                }
            })
            .collect()
    }

    #[must_use]
    pub fn content_line_count(&self) -> usize {
        self.content_lines().len()
    }

    #[must_use]
    pub fn get_subject(&self) -> String {
        self.content_lines()
            .get(0)
            .map(String::from)
            .unwrap_or_default()
    }

    #[must_use]
    pub fn get_body(&self) -> String {
        self.content_lines()
            .into_iter()
            .skip(1)
            .skip_while(std::string::String::is_empty)
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[must_use]
    pub fn get_trailer(&self, trailer_key: &str) -> Vec<Trailer> {
        self.get_all_trailers()
            .into_iter()
            .filter(|trailer| trailer.has_key(trailer_key))
            .collect()
    }

    fn get_all_trailers(&self) -> Vec<Trailer> {
        self.contents
            .lines()
            .filter_map(|line: &str| Trailer::from_str(line).ok())
            .collect()
    }

    #[must_use]
    pub fn add_trailer(&self, trailer: &Trailer) -> Self {
        let (body, tail) = self.message_parts();

        if self.has_line(&trailer.to_string()) {
            self.clone()
        } else if body.is_empty() && tail.is_empty() {
            Self::new(format!("\n{}\n", trailer))
        } else if body.is_empty() {
            Self::new(format!("\n{}\n\n{}\n", trailer, tail))
        } else if tail.is_empty() {
            Self::new(format!("{}\n\n{}\n", body, trailer))
        } else {
            Self::new(format!("{}\n{}\n\n{}\n", body, trailer, tail))
        }
    }

    fn has_line(&self, search: &str) -> bool {
        self.contents
            .lines()
            .map(|line| line)
            .any(|line| line == search)
    }

    fn message_parts(&self) -> (String, String) {
        let contents_length = self.lines().count();
        let tail_length = self.tail_length();
        let body_length = contents_length - tail_length;

        let body: Vec<&str> = self.lines().take(body_length).collect();
        let tail: Vec<&str> = self.lines().skip(body_length).collect();

        (body.join("\n"), tail.join("\n"))
    }

    fn tail_length(&self) -> usize {
        let scissors_section_length = self.scissors_section_length();

        let reverse_comments_section: Vec<&str> = self
            .lines()
            .rev()
            .skip(scissors_section_length)
            .take_while(|line| line.starts_with(&self.comment_char) || line.is_empty())
            .collect();

        let comments_section = reverse_comments_section.iter().rev();

        let blank_lines = comments_section
            .clone()
            .take_while(|line| line.is_empty())
            .count();

        let comments_section_length = comments_section.count();

        scissors_section_length + comments_section_length - blank_lines
    }

    fn scissors_section_length(&self) -> usize {
        let scissors_line = self.scissors_line();

        if self.lines().any(|line| *line == scissors_line) {
            self.lines()
                .rev()
                .take_while(|line| *line != scissors_line)
                .count()
        } else {
            0
        }
    }

    fn scissors_line(&self) -> String {
        let scissors_line = format!("{} {}", self.comment_char, SCISSORS_LINE);
        scissors_line
    }

    fn lines(&self) -> Lines {
        self.contents.lines()
    }
}

fn detect_comment_char(contents: &str) -> &str {
    let pattern = RegexBuilder::new(&format!("^(?P<char>[^\\s]+) {}$", SCISSORS_LINE))
        .multi_line(true)
        .build()
        .unwrap();

    pattern
        .captures(contents)
        .and_then(|caps| caps.name("char"))
        .map_or("#", |m| m.as_str())
}

impl TryFrom<PathBuf> for CommitMessage {
    type Error = Error;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let mut file = File::open(value)?;
        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .map_err(Error::from)
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
    use std::str::FromStr;

    use indoc::indoc;

    use super::{CommitMessage, Trailer};

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
            .into(),
        );

        assert_eq!(
            vec![Trailer::new("Another", "Trailer")],
            commit.get_trailer("Another")
        );
        assert_eq!(
            vec![
                Trailer::new("Anything", "Some Trailer"),
                Trailer::new("Anything", "Some Trailer"),
            ],
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
            CommitMessage::new("".into())
                .add_trailer(&Trailer::from_str("Anything: Some Trailer").unwrap())
        );
    }

    #[test]
    fn adding_trailer_simple_message() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "
                    Simple commit message

                    With a description.

                    Anything: Some Trailer
                    "
                )
                .into(),
            ),
            CommitMessage::new(
                indoc!(
                    "
                    Simple commit message

                    With a description.
                    "
                )
                .into(),
            )
            .add_trailer(&Trailer::from_str("Anything: Some Trailer").unwrap())
        );
    }

    #[test]
    fn adding_a_trailer_twice() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "
                    Simple commit message

                    With a description.

                    Anything: Some Trailer
                    "
                )
                .into(),
            ),
            CommitMessage::new(
                indoc!(
                    "
                    Simple commit message

                    With a description.

                    Anything: Some Trailer
                    "
                )
                .into(),
            )
            .add_trailer(&Trailer::from_str("Anything: Some Trailer").unwrap())
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
                .into(),
            ),
            CommitMessage::new("# Comments about writing a commit message\n".into())
                .add_trailer(&Trailer::from_str("Trailer: Title").unwrap())
        );
    }

    #[test]
    fn adding_trailer_when_message_contains_content_with_trailing_comments() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    Trailer: Title

                    # Comment about committing
                    "
                )
                .into(),
            ),
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    # Comment about committing
                    "
                )
                .into(),
            )
            .add_trailer(&Trailer::from_str("Trailer: Title").unwrap())
        );
    }

    #[test]
    fn adding_trailer_when_there_are_additional_comments() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    # Random Comment

                    Message content

                    Trailer: Title

                    # Trailing comment line 1
                    # Trailing comment line 2
                    "
                )
                .into(),
            ),
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    # Random Comment

                    Message content

                    # Trailing comment line 1
                    # Trailing comment line 2
                    "
                )
                .into(),
            )
            .add_trailer(&Trailer::from_str("Trailer: Title").unwrap())
        );
    }

    #[test]
    fn adding_trailer_when_is_a_scissors_line() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    Trailer: Content

                    # On branch main
                    # Your branch is ahead of 'origin/main' by 18 commits.
                    #   (use \"git push\" to publish your local commits)
                    #
                    # Changes to be committed:
                    #	modified:   commit_message.rs
                    #
                    # ------------------------ >8 ------------------------
                    # Do not modify or remove the line above.
                    # Everything below it will be ignored.
                    diff --git a/commit_message.rs b/commit_message.rs
                    ...
                    "
                )
                .into(),
            ),
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    # On branch main
                    # Your branch is ahead of 'origin/main' by 18 commits.
                    #   (use \"git push\" to publish your local commits)
                    #
                    # Changes to be committed:
                    #	modified:   commit_message.rs
                    #
                    # ------------------------ >8 ------------------------
                    # Do not modify or remove the line above.
                    # Everything below it will be ignored.
                    diff --git a/commit_message.rs b/commit_message.rs
                    ...
                    "
                )
                .into(),
            )
            .add_trailer(&Trailer::from_str("Trailer: Content").unwrap())
        );
    }

    #[test]
    fn adding_trailer_with_different_comment_character() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    Trailer: Content

                    % On branch main
                    % Your branch is ahead of 'origin/main' by 18 commits.
                    %   (use \"git push\" to publish your local commits)
                    %
                    % Changes to be committed:
                    %	modified:   commit_message.rs
                    %
                    % ------------------------ >8 ------------------------
                    % Do not modify or remove the line above.
                    % Everything below it will be ignored.
                    diff --git a/commit_message.rs b/commit_message.rs
                    "
                )
                .into(),
            ),
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    % On branch main
                    % Your branch is ahead of 'origin/main' by 18 commits.
                    %   (use \"git push\" to publish your local commits)
                    %
                    % Changes to be committed:
                    %	modified:   commit_message.rs
                    %
                    % ------------------------ >8 ------------------------
                    % Do not modify or remove the line above.
                    % Everything below it will be ignored.
                    diff --git a/commit_message.rs b/commit_message.rs
                    "
                )
                .into(),
            )
            .add_trailer(&Trailer::from_str("Trailer: Content").unwrap())
        );
    }

    #[test]
    fn adding_trailer_when_message_template_is_present() {
        assert_eq!(
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    Trailer: Content

                    # Template message line 1
                    # Template message line 2

                    # On branch main
                    # Your branch is ahead of 'origin/main' by 18 commits.
                    #   (use \"git push\" to publish your local commits)
                    #
                    # Changes to be committed:
                    #	modified:   commit_message.rs
                    #
                    # ------------------------ >8 ------------------------
                    # Do not modify or remove the line above.
                    # Everything below it will be ignored.
                    diff --git a/commit_message.rs b/commit_message.rs
                    "
                )
                .into(),
            ),
            CommitMessage::new(
                indoc!(
                    "
                    Message title

                    # Template message line 1
                    # Template message line 2

                    # On branch main
                    # Your branch is ahead of 'origin/main' by 18 commits.
                    #   (use \"git push\" to publish your local commits)
                    #
                    # Changes to be committed:
                    #	modified:   commit_message.rs
                    #
                    # ------------------------ >8 ------------------------
                    # Do not modify or remove the line above.
                    # Everything below it will be ignored.
                    diff --git a/commit_message.rs b/commit_message.rs
                    "
                )
                .into(),
            )
            .add_trailer(&Trailer::from_str("Trailer: Content").unwrap())
        );
    }

    #[test]
    fn can_get_only_subject() {
        let commit = CommitMessage::new(
            indoc!(
                "
                Some Commit Message
                "
            )
            .into(),
        );
        assert_eq!("Some Commit Message", commit.get_subject());
    }

    #[test]
    fn can_get_a_commented_out_subject() {
        // You have to pass the "allow empty commit" flag o git to achieve this
        let commit = CommitMessage::new(
            indoc!(
                "
                # Some Commit Message
                "
            )
            .into(),
        );
        assert_eq!("", commit.get_subject());
    }

    #[test]
    fn can_get_a_subject_with_a_first_comment_line() {
        // You have to pass the "allow empty commit" flag o git to achieve this
        let commit = CommitMessage::new(
            indoc!(
                "
                # Some Commit Message
                The subject
                "
            )
            .into(),
        );
        assert_eq!("The subject", commit.get_subject());
    }

    #[test]
    fn can_line_count() {
        let commit = CommitMessage::new(
            indoc!(
                "
                Some Commit Message

                With a description.
                "
            )
            .into(),
        );
        assert_eq!(3, commit.content_line_count());
    }

    #[test]
    fn line_count_does_not_include_comments() {
        let commit = CommitMessage::new(
            indoc!(
                "
                Some Commit Message

                # I am not counted
                With a description.
                And more content
                "
            )
            .into(),
        );
        assert_eq!(4, commit.content_line_count());
    }

    #[test]
    fn line_count_does_not_include_comments_anything_after_scissors() {
        let commit = CommitMessage::new(
            indoc!(
                "
                Some Commit Message

                # I am not counted
                With a description.
                And more content

                # On branch main
                # Your branch is ahead of 'origin/main' by 18 commits.
                #   (use \"git push\" to publish your local commits)
                #
                # Changes to be committed:
                #	modified:   commit_message.rs
                #
                # ------------------------ >8 ------------------------
                # Do not modify or remove the line above.
                # Everything below it will be ignored.
                diff --git a/commit_message.rs b/commit_message.rs
                "
            )
            .into(),
        );
        assert_eq!(5, commit.content_line_count());
    }

    #[test]
    fn can_get_body() {
        let commit = CommitMessage::new(
            indoc!(
                "
                Some Commit Message

                With a description.
                "
            )
            .into(),
        );
        assert_eq!("With a description.".to_string(), commit.get_body());
    }

    #[test]
    fn body_does_not_include_comments() {
        let commit = CommitMessage::new(
            indoc!(
                "
                Some Commit Message

                # I am not counted
                With a description.
                And more content
                "
            )
            .into(),
        );
        assert_eq!(
            indoc!(
                "With a description.
                And more content"
            )
            .to_string(),
            commit.get_body()
        );
    }

    #[test]
    fn body_does_not_include_comments_anything_after_scissors() {
        let commit = CommitMessage::new(
            indoc!(
                "
                Some Commit Message

                # I am not counted
                With a description.
                And more content

                # On branch main
                # Your branch is ahead of 'origin/main' by 18 commits.
                #   (use \"git push\" to publish your local commits)
                #
                # Changes to be committed:
                #	modified:   commit_message.rs
                #
                # ------------------------ >8 ------------------------
                # Do not modify or remove the line above.
                # Everything below it will be ignored.
                diff --git a/commit_message.rs b/commit_message.rs
                "
            )
            .into(),
        );
        assert_eq!(
            indoc!(
                "With a description.
                And more content
                "
            )
            .to_string(),
            commit.get_body()
        );
    }
    #[test]
    fn body_still_retrieved_without_gutter() {
        let commit = CommitMessage::new(
            indoc!(
                "Some Commit Message
                With a description."
            )
            .into(),
        );
        assert_eq!("With a description.".to_string(), commit.get_body());
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read commit message from file: {0}")]
    CommitMessageRead(#[from] io::Error),
}
