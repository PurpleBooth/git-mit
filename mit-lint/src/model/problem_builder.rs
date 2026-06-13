use miette::SourceOffset;
use mit_commit::CommitMessage;

use crate::model::{Code, Problem};

/// Builder for creating Problem instances with a fluent interface
pub struct ProblemBuilder {
    error: String,
    tip: String,
    code: Code,
    commit_message: String,
    labels: Vec<(String, usize, usize)>,
    url: Option<String>,
}

impl ProblemBuilder {
    /// Create a new problem builder with required fields
    ///
    /// # Arguments
    ///
    /// * `error` - The error message
    /// * `tip` - Advice on how to fix the problem
    /// * `code` - The error code
    /// * `commit_message` - The commit message that has the problem
    ///
    /// # Returns
    ///
    /// A new `ProblemBuilder` instance
    pub fn new(
        error: impl Into<String>,
        tip: impl Into<String>,
        code: Code,
        commit_message: &CommitMessage<'_>,
    ) -> Self {
        Self {
            error: error.into(),
            tip: tip.into(),
            code,
            commit_message: String::from(commit_message.clone()),
            labels: Vec::new(),
            url: None,
        }
    }

    /// Add a URL with more information about the problem
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to documentation about this problem
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Add a label to highlight a specific part of the commit message
    ///
    /// # Arguments
    ///
    /// * `text` - The label text
    /// * `position` - The byte offset in the commit message
    /// * `length` - The length of the highlighted section in bytes
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_label(mut self, text: impl Into<String>, position: usize, length: usize) -> Self {
        self.labels.push((text.into(), position, length));
        self
    }

    /// Add a label for a line that exceeds a character limit
    ///
    /// # Arguments
    ///
    /// * `commit_text` - The full commit message text
    /// * `line_index` - The zero-based index of the line
    /// * `line` - The content of the line
    /// * `limit` - The character limit
    /// * `label_text` - The text to show in the label
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_label_for_line(
        self,
        commit_text: &str,
        line_index: usize,
        line: &str,
        limit: usize,
        label_text: impl Into<String>,
    ) -> Self {
        if line.chars().count() <= limit {
            return self;
        }

        let position = SourceOffset::from_location(
            commit_text,
            line_index + 1,
            line.chars().take(limit).map(char::len_utf8).sum::<usize>() + 1,
        )
        .offset();

        let length = line.chars().count().saturating_sub(limit);

        self.with_label(label_text, position, length)
    }

    /// Add a label for the last non-empty line of the commit message.
    ///
    /// This is useful for indicating that something is missing at the end of the commit message.
    ///
    /// # Arguments
    ///
    /// * `label_text` - The text to show in the label
    pub fn with_label_at_last_line(self, label_text: impl Into<String>) -> Self {
        let original = &self.commit_message;
        let trimmed = original.trim_end();
        let trimmed_len = trimmed.len();

        // Find the last newline in the trimmed string
        let last_line_start = trimmed.rfind('\n').map_or(0, |pos| pos + 1);
        let last_line_length = trimmed_len - last_line_start;
        self.with_label(label_text, last_line_start, last_line_length)
    }

    /// Build the Problem instance
    ///
    /// # Returns
    ///
    /// A Problem instance with all the configured properties
    pub fn build(self) -> Problem {
        let labels = if self.labels.is_empty() {
            None
        } else {
            Some(self.labels)
        };

        Problem::new(
            self.error,
            self.tip,
            self.code,
            &self.commit_message.clone().into(),
            labels,
            self.url,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miette::Diagnostic;
    use mit_commit::CommitMessage;

    #[test]
    fn test_builder_creates_problem_with_basic_fields() {
        let commit = CommitMessage::from("Test commit");
        let problem = ProblemBuilder::new(
            "Error message",
            "Fix advice",
            Code::BodyWiderThan72Characters,
            &commit,
        )
        .build();

        assert_eq!(problem.error(), "Error message");
        assert_eq!(problem.tip(), "Fix advice");
        assert_eq!(problem.code(), &Code::BodyWiderThan72Characters);
        assert_eq!(problem.commit_message(), commit);
    }

    #[test]
    fn test_builder_adds_url() {
        let commit = CommitMessage::from("Test commit");
        let problem = ProblemBuilder::new(
            "Error message",
            "Fix advice",
            Code::BodyWiderThan72Characters,
            &commit,
        )
        .with_url("https://example.com")
        .build();

        // We can't directly access the URL, but we can check the diagnostic output
        let diagnostic_output = format!("{problem:?}");
        assert!(diagnostic_output.contains("https://example.com"));
    }

    #[test]
    fn test_builder_adds_labels() {
        let commit = CommitMessage::from("Test commit");
        let problem = ProblemBuilder::new(
            "Error message",
            "Fix advice",
            Code::BodyWiderThan72Characters,
            &commit,
        )
        .with_label("Label 1", 0, 4)
        .with_label("Label 2", 5, 6)
        .build();

        // We can't directly access the labels, but we can check the diagnostic output
        let diagnostic_output = format!("{problem:?}");
        assert!(diagnostic_output.contains("Label 1"));
        assert!(diagnostic_output.contains("Label 2"));
    }

    #[test]
    fn test_with_label_for_line() {
        let commit_text = "Subject\n\nThis is a very long line that exceeds the character limit";
        let commit = CommitMessage::from(commit_text);

        let problem = ProblemBuilder::new(
            "Error message",
            "Fix advice",
            Code::BodyWiderThan72Characters,
            &commit,
        )
        .with_label_for_line(
            commit_text,
            2,
            "This is a very long line that exceeds the character limit",
            10,
            "Too long",
        )
        .build();

        // We can't directly access the labels, but we can check the diagnostic output
        let diagnostic_output = format!("{problem:?}");
        assert!(diagnostic_output.contains("Too long"));
    }

    #[test]
    fn test_with_label_for_line_does_not_add_label_if_within_limit() {
        let commit_text = "Subject\n\nShort line";
        let commit = CommitMessage::from(commit_text);

        let problem = ProblemBuilder::new(
            "Error message",
            "Fix advice",
            Code::BodyWiderThan72Characters,
            &commit,
        )
        .with_label_for_line(commit_text, 2, "Short line", 72, "Too long")
        .build();

        // Check that no labels were added
        let diagnostic_output = format!("{problem:?}");
        assert!(!diagnostic_output.contains("Too long"));
    }

    #[test]
    fn test_builder_with_multiple_methods_chained() {
        let commit_text = "Subject\n\nThis is a very long line that exceeds the character limit";
        let commit = CommitMessage::from(commit_text);

        let problem = ProblemBuilder::new(
            "Error message",
            "Fix advice",
            Code::BodyWiderThan72Characters,
            &commit,
        )
        .with_url("https://example.com")
        .with_label("Manual label", 0, 7)
        .with_label_for_line(
            commit_text,
            2,
            "This is a very long line that exceeds the character limit",
            10,
            "Too long",
        )
        .build();

        let diagnostic_output = format!("{problem:?}");
        assert!(diagnostic_output.contains("https://example.com"));
        assert!(diagnostic_output.contains("Manual label"));
        assert!(diagnostic_output.contains("Too long"));
    }

    #[test]
    fn test_with_label_for_line_position_calculation() {
        // Setup a multi-line commit message with known positions
        let commit_text = "Subject\nSecond line\nThis is a line that exceeds the limit";
        let commit = CommitMessage::from(commit_text);

        // The line we're testing is the third line (index 2)
        let line_index = 2;
        let line = "This is a line that exceeds the limit";
        let limit = 10;

        // Create a problem with a label at the position where the line exceeds the limit
        let problem = ProblemBuilder::new(
            "Error message",
            "Fix advice",
            Code::BodyWiderThan72Characters,
            &commit,
        )
        .with_label_for_line(commit_text, line_index, line, limit, "Too long")
        .build();

        // Calculate the expected position - the start of the third line plus the first 10 characters
        let third_line_offset = commit_text.find("This is a line").unwrap();
        let first_ten_chars_length = line.chars().take(limit).map(char::len_utf8).sum::<usize>();
        let expected_position = third_line_offset + first_ten_chars_length;

        // Verify the label is positioned correctly
        let labels = problem.labels().unwrap().collect::<Vec<_>>();
        assert_eq!(
            labels[0].offset(),
            expected_position,
            "Label should be positioned at character {} (after the first {} characters of line {})",
            expected_position,
            limit,
            line_index + 1
        );
    }
}
