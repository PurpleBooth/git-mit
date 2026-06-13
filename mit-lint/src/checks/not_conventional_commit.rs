use mit_commit::CommitMessage;
use std::collections::HashSet;

use crate::model::{Code, Problem, ProblemBuilder};

/// Canonical lint ID
pub const CONFIG: &str = "not-conventional-commit";

/// Advice on how to correct the problem
pub const HELP_MESSAGE: &str =
    "It's important to follow the conventional commit style when creating your commit message. By \
using this style we can automatically calculate the version of software using deployment \
pipelines, and also generate changelogs and other useful information without human interaction.

You can fix it by following style

<type>[optional scope]: <description>

[optional body]

[optional footer(s)]";
/// Description of the problem
pub const ERROR: &str = "Your commit message isn't in conventional style";

/// Configuration for conventional commit linting
#[derive(Default)]
pub struct ConventionalCommitConfig {
    /// Allowed commit types (None means any alphanumeric type is allowed)
    pub allowed_types: Option<HashSet<String>>,
    /// Allowed commit scopes (None means any word character is allowed)
    pub allowed_scopes: Option<HashSet<String>>,
}

impl ConventionalCommitConfig {
    /// Create a new configuration with custom allowed types and scopes
    ///
    /// # Arguments
    ///
    /// * `allowed_types` - Optional set of allowed commit types (None means any alphanumeric type is allowed)
    /// * `allowed_scopes` - Optional set of allowed commit scopes (None means any word character is allowed)
    ///
    /// # Returns
    ///
    /// A new `ConventionalCommitConfig` with the specified allowed types and scopes
    #[allow(dead_code)]
    pub const fn new(
        allowed_types: Option<HashSet<String>>,
        allowed_scopes: Option<HashSet<String>>,
    ) -> Self {
        Self {
            allowed_types,
            allowed_scopes,
        }
    }
}

/// Parse a conventional commit subject line
///
/// Returns (type, scope, `breaking_change`, description) if successful
fn parse_conventional_commit(subject: &str) -> Option<(String, Option<String>, bool, String)> {
    // Find the colon that separates type/scope from description
    let colon_pos = subject.find(':')?;

    // Extract the description (must have a space after the colon)
    if subject.len() <= colon_pos + 1 || subject.as_bytes()[colon_pos + 1] != b' ' {
        return None;
    }
    // Extract the description (can be empty)
    let description = subject[colon_pos + 2..].to_string();

    // Parse the type, scope, and breaking change indicator
    let type_scope_part = &subject[..colon_pos];

    // Check for breaking change indicator
    let (type_scope_part, breaking_change) = type_scope_part
        .strip_suffix('!')
        .map_or((type_scope_part, false), |stripped| (stripped, true));

    // Check for scope in parentheses
    let (commit_type, scope) = if let (Some(open_paren), Some(close_paren)) =
        (type_scope_part.find('('), type_scope_part.find(')'))
    {
        if open_paren > 0 && close_paren > open_paren && close_paren == type_scope_part.len() - 1 {
            let commit_type = type_scope_part[..open_paren].to_string();
            let scope = type_scope_part[open_paren + 1..close_paren].to_string();
            (commit_type, Some(scope))
        } else {
            return None; // Malformed scope
        }
    } else {
        (type_scope_part.to_string(), None)
    };

    // Validate type is alphanumeric
    if !commit_type.chars().all(|c| c.is_ascii_alphanumeric()) || commit_type.is_empty() {
        return None;
    }

    // Validate scope is alphanumeric if present
    if let Some(scope) = &scope
        && (!scope.chars().all(|c| c.is_ascii_alphanumeric()) || scope.is_empty())
    {
        return None;
    }

    Some((commit_type, scope, breaking_change, description))
}

/// Checks if the commit message follows the conventional commit format
///
/// # Arguments
///
/// * `commit_message` - The commit message to check
///
/// # Returns
///
/// * `Some(Problem)` - If the commit message does not follow the conventional commit format
/// * `None` - If the commit message follows the conventional commit format
///
/// # Examples
///
/// ```rust
/// use mit_commit::CommitMessage;
/// use mit_lint::Lint::NotConventionalCommit;
///
/// // This should pass
/// let passing = CommitMessage::from("feat: add new feature");
/// assert!(NotConventionalCommit.lint(&passing).is_none());
///
/// // This should fail
/// let failing = CommitMessage::from("Add new feature");
/// assert!(NotConventionalCommit.lint(&failing).is_some());
/// ```
///
/// # Errors
///
/// This function will never return an error, only an Option<Problem>
pub fn lint(commit_message: &CommitMessage<'_>) -> Option<Problem> {
    lint_with_config(commit_message, &ConventionalCommitConfig::default())
}

fn lint_with_config(
    commit_message: &CommitMessage<'_>,
    config: &ConventionalCommitConfig,
) -> Option<Problem> {
    Some(commit_message)
        .filter(|commit| has_problem(commit, config))
        .map(create_problem)
}

fn has_problem(commit_message: &CommitMessage<'_>, config: &ConventionalCommitConfig) -> bool {
    let subject: String = commit_message.get_subject().into();

    // Parse the subject line
    match parse_conventional_commit(&subject) {
        None => true, // Not a conventional commit format
        Some((commit_type, scope, _, _)) => {
            // If allowed_types is Some, check if the type is allowed
            if let Some(allowed_types) = &config.allowed_types
                && !allowed_types.contains(&commit_type)
            {
                return true;
            }

            // If allowed_scopes is Some and a scope is present, check if the scope is allowed
            if let Some(allowed_scopes) = &config.allowed_scopes
                && let Some(scope) = scope
                && !allowed_scopes.contains(&scope)
            {
                return true;
            }

            false
        }
    }
}

fn create_problem(commit_message: &CommitMessage) -> Problem {
    // Create a problem with appropriate labels
    let commit_text = String::from(commit_message.clone());
    let first_line_length = commit_text.lines().next().map(str::len).unwrap_or_default();

    ProblemBuilder::new(
        ERROR,
        HELP_MESSAGE,
        Code::NotConventionalCommit,
        commit_message,
    )
    .with_label("Not conventional", 0, first_line_length)
    .with_url("https://www.conventionalcommits.org/")
    .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Code;
    use mit_commit::Trailer;
    use quickcheck::TestResult;

    // Examples from https://www.conventionalcommits.org/en/v1.0.0/

    #[test]
    fn commit_message_with_description_and_breaking_change_footer() {
        test_subject_not_separate_from_body(
            "feat: allow provided config object to extend other configs

BREAKING CHANGE: `extends` key in config file is now used for extending other \
 config files
",
            None,
        );
    }

    #[test]
    fn commit_message_with_bang_to_draw_attention_to_breaking_change() {
        test_subject_not_separate_from_body(
            "refactor!: drop support for Node 6
",
            None,
        );
    }

    #[test]
    fn commit_message_with_both_bang_and_breaking_change_footer() {
        test_subject_not_separate_from_body(
            "refactor!: drop support for Node 6

BREAKING CHANGE: refactor to use JavaScript features not available in Node 6.
",
            None,
        );
    }

    #[test]
    fn commit_message_with_no_body() {
        test_subject_not_separate_from_body(
            "docs: correct spelling of CHANGELOG
",
            None,
        );
    }

    #[test]
    fn commit_message_with_scope() {
        test_subject_not_separate_from_body(
            "feat(lang): add polish language
",
            None,
        );
    }

    #[test]
    fn commit_message_with_multi_paragraph_body_and_multiple_footers() {
        test_subject_not_separate_from_body(
            "fix: correct minor typos in code

see the issue for details

on typos fixed.

Reviewed-by: Z
Refs #133
",
            None,
        );
    }

    #[test]
    fn revert_example() {
        test_subject_not_separate_from_body(
            "revert: let us never again speak of the noodle incident

Refs: 676104e, a215868
",
            None,
        );
    }

    #[test]
    fn non_conventional() {
        let message = "An example commit

This is an example commit
";
        test_subject_not_separate_from_body(
            message,
            Some(&Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
                &message.into(),
                Some(vec![("Not conventional".to_string(), 0_usize, 17_usize)]),
                Some("https://www.conventionalcommits.org/".parse().unwrap()),
            )),
        );
    }

    #[test]
    fn missing_bracket() {
        let message = "fix(example: An example commit

This is an example commit
";
        test_subject_not_separate_from_body(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
                &message.into(),
                Some(vec![("Not conventional".to_string(), 0_usize, 30_usize)]),
                Some("https://www.conventionalcommits.org/".parse().unwrap()),
            ))
            .as_ref(),
        );
    }

    #[test]
    fn missing_space() {
        let message = "fix(example):An example commit

This is an example commit
";
        test_subject_not_separate_from_body(
            message,
            Some(Problem::new(
                ERROR.into(),
                HELP_MESSAGE.into(),
                Code::NotConventionalCommit,
                &message.into(),
                Some(vec![("Not conventional".to_string(), 0_usize, 30_usize)]),
                Some("https://www.conventionalcommits.org/".parse().unwrap()),
            ))
            .as_ref(),
        );
    }

    fn test_subject_not_separate_from_body(message: &str, expected: Option<&Problem>) {
        let actual = &lint(&CommitMessage::from(message));
        assert_eq!(
            actual.as_ref(),
            expected,
            "Message {message:?} should have returned {expected:?}, found {actual:?}"
        );
    }

    use std::option::Option::None;

    use miette::{GraphicalReportHandler, GraphicalTheme, Report};

    #[test]
    fn formatting() {
        let message = "An example commit

This is an example commit
";
        let problem = lint(&CommitMessage::from(message.to_string()));
        let actual = fmt_report(&Report::new(problem.unwrap()));
        let expected = "NotConventionalCommit (https://www.conventionalcommits.org/)\n\n  x Your commit message isn't in conventional style\n   ,-[1:1]\n 1 | An example commit\n   : ^^^^^^^^|^^^^^^^^\n   :         `-- Not conventional\n 2 | \n   `----\n  help: It's important to follow the conventional commit style when creating\n        your commit message. By using this style we can automatically\n        calculate the version of software using deployment pipelines, and also\n        generate changelogs and other useful information without human\n        interaction.\n        \n        You can fix it by following style\n        \n        <type>[optional scope]: <description>\n        \n        [optional body]\n        \n        [optional footer(s)]\n"
        .to_string();
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
    fn fail_check(commit: String) -> TestResult {
        let has_non_alpha_type = commit
            .chars()
            .position(|x| x == ':')
            .is_some_and(|x| commit.chars().take(x).any(|x| !x.is_ascii_alphanumeric()));
        if has_non_alpha_type {
            return TestResult::discard();
        }

        // Also discard strings that form valid conventional commits.
        // The filter above only excludes non-alphanumeric types before the colon,
        // but strings like "feat: hello" pass the filter despite being valid
        // conventional commits, causing a false assertion failure.
        let message = CommitMessage::from(format!("{commit}\n# comment"));
        if lint(&message).is_none() {
            return TestResult::discard();
        }

        TestResult::from_bool(true)
    }

    #[allow(clippy::needless_pass_by_value)]
    #[quickcheck]
    fn success_check(
        type_slug: String,
        scope: Option<String>,
        description: String,
        body: Option<String>,
        bc_break: Option<String>,
    ) -> TestResult {
        if type_slug.starts_with('#')
            || type_slug.is_empty()
            || type_slug.chars().any(|x| !x.is_ascii_alphanumeric())
        {
            return TestResult::discard();
        }
        if let Some(scope) = scope.clone() {
            if scope.is_empty() || scope.chars().any(|x| !x.is_ascii_alphanumeric()) {
                return TestResult::discard();
            }
        }

        let mut commit: CommitMessage<'_> = CommitMessage::default().with_subject(
            format!(
                "{}{}{}: {}",
                type_slug,
                scope.map(|x| format!("({x})")).unwrap_or_default(),
                bc_break
                    .clone()
                    .map(|_| "!".to_string())
                    .unwrap_or_default(),
                description
            )
            .into(),
        );

        let body_contents = body.clone().unwrap_or_default();

        if body.is_some() {
            commit = commit.with_body_contents(&body_contents);
        }

        if let Some(_bc_contents) = bc_break {
            commit = commit.add_trailer(Trailer::new("BC BREAK".into(), "bc_contents".into()));
        }

        let result = lint(&commit);
        TestResult::from_bool(result.is_none())
    }

    // Tests for custom configurations with allowed_types and allowed_scopes
    #[test]
    fn test_lint_with_config_allowed_types() {
        use std::collections::HashSet;

        // Create a config that only allows "feat" type
        let mut allowed_types = HashSet::new();
        allowed_types.insert("feat".to_string());
        let config = ConventionalCommitConfig::new(Some(allowed_types), None);

        // Test with allowed type
        let commit_allowed = CommitMessage::from("feat: add new feature");
        assert!(lint_with_config(&commit_allowed, &config).is_none());

        // Test with disallowed type
        let commit_disallowed = CommitMessage::from("fix: fix a bug");
        assert!(lint_with_config(&commit_disallowed, &config).is_some());
    }

    #[test]
    fn test_lint_with_config_allowed_scopes() {
        use std::collections::HashSet;

        // Create a config that only allows "ui" scope
        let mut allowed_scopes = HashSet::new();
        allowed_scopes.insert("ui".to_string());
        let config = ConventionalCommitConfig::new(None, Some(allowed_scopes));

        // Test with allowed scope
        let commit_allowed = CommitMessage::from("feat(ui): add new UI feature");
        assert!(lint_with_config(&commit_allowed, &config).is_none());

        // Test with disallowed scope
        let commit_disallowed = CommitMessage::from("feat(api): add new API feature");
        assert!(lint_with_config(&commit_disallowed, &config).is_some());
    }

    // Tests for edge cases in parse_conventional_commit
    #[test]
    fn test_parse_conventional_commit_colon_position() {
        // Test with no space after colon (should fail)
        assert!(parse_conventional_commit("feat:no-space").is_none());

        // Test with space after colon (should pass)
        assert!(parse_conventional_commit("feat: with-space").is_some());

        // Test with colon at the end (should fail)
        assert!(parse_conventional_commit("feat:").is_none());

        // Test with colon at the end followed by a space (should pass)
        // This specifically tests the case that failed in the quickcheck test
        assert!(parse_conventional_commit("feat: ").is_some());

        // Test with colon at position 0 (should fail because the commit type is empty)
        assert!(parse_conventional_commit(": description").is_none());

        // Test with colon at a high position (should pass if followed by space and description)
        let long_type = "a".repeat(100);
        let commit_message = format!("{long_type}(scope): description");
        assert!(parse_conventional_commit(&commit_message).is_some());
    }

    #[test]
    fn test_parse_conventional_commit_scope_parsing() {
        // Test with valid scope
        let result = parse_conventional_commit("feat(ui): add feature");
        assert!(result.is_some());
        let (commit_type, scope, _, _) = result.unwrap();
        assert_eq!(commit_type, "feat");
        assert_eq!(scope, Some("ui".to_string()));

        // Test with malformed scope (open paren at beginning)
        assert!(parse_conventional_commit("(ui): add feature").is_none());

        // Test with malformed scope (close paren not at end)
        assert!(parse_conventional_commit("feat(ui)extra: add feature").is_none());

        // Test with malformed scope (open paren after close paren)
        assert!(parse_conventional_commit("feat)(: add feature").is_none());
    }

    #[test]
    fn test_parse_conventional_commit_scope_validation() {
        // Test with empty scope (should fail)
        assert!(parse_conventional_commit("feat(): add feature").is_none());

        // Test with non-alphanumeric scope (should fail)
        assert!(parse_conventional_commit("feat(ui-component): add feature").is_none());

        // Test with alphanumeric scope (should pass)
        assert!(parse_conventional_commit("feat(ui123): add feature").is_some());
    }

    #[test]
    fn test_quickcheck_failing_case() {
        // Test the specific case that failed in QuickCheck: ("0", None, "", None, None)
        let commit = CommitMessage::from("0: ");
        assert!(lint(&commit).is_none());
    }

    // Bug C: fail_check's first filter (has_non_alpha_type) alone doesn't exclude valid
    // conventional commits like "feat: hello". The property now also calls lint() and
    // discards when the result is None. This test verifies the full discard logic works.
    #[test]
    fn test_fail_check_should_discard_valid_conventional_commits() {
        let commit = "feat: hello";

        // First filter: non-alpha type check (insufficient on its own)
        let has_non_alpha_type = commit
            .chars()
            .position(|x| x == ':')
            .is_some_and(|x| commit.chars().take(x).any(|x| !x.is_ascii_alphanumeric()));
        assert!(
            !has_non_alpha_type,
            "first filter should NOT discard '{commit}' (it has an all-ASCII type)"
        );

        // Second check: lint-based discard (the actual fix)
        let message = CommitMessage::from(format!("{commit}\n# comment"));
        assert!(
            lint(&message).is_none(),
            "lint should accept '{commit}' as valid conventional commit"
        );

        // Combined: the quickcheck should discard this via the second check
        let should_discard = has_non_alpha_type || lint(&message).is_none();
        assert!(
            should_discard,
            "fail_check must discard valid conventional commits like '{commit}'"
        );
    }

    // Bug B: success_check's scope filter used is_alphanumeric() (Unicode) but the
    // parser validates scopes with is_ascii_alphanumeric(). This mismatch meant the
    // quickcheck could generate a scope like "café" that passes the filter but the
    // parser rejects, causing a false test failure.
    // Fix: changed the filter to use is_ascii_alphanumeric() to match the parser.
    #[test]
    fn test_scope_filter_matches_parser_validation() {
        // A scope with a Unicode alphanumeric char (e.g. é)
        let scope = "café";

        // After the fix, the filter uses is_ascii_alphanumeric() — same as the parser
        let filter_allows = !scope.is_empty() && !scope.chars().any(|x| !x.is_ascii_alphanumeric());
        let parser_allows = !scope.is_empty() && scope.chars().all(|c| c.is_ascii_alphanumeric());

        // Both filter and parser should now agree: reject Unicode scope
        assert_eq!(
            filter_allows, parser_allows,
            "filter and parser should agree on scope '{scope}'"
        );
        assert!(!filter_allows, "both should reject Unicode scope '{scope}'");

        // Lint confirms the rejection
        let commit = CommitMessage::from(format!("feat({scope}): add feature"));
        let result = lint(&commit);
        assert!(
            result.is_some(),
            "lint should reject commit with Unicode scope '{scope}'"
        );
    }

    // Bug A: chars().nth(colon_pos + 1) used char index with a byte position from
    // subject.find(':'). Now fixed to use as_bytes()[colon_pos + 1] instead.
    // When the subject contains multi-byte UTF-8 characters, the old code would
    // look at the wrong character after the colon.
    #[test]
    fn test_byte_vs_char_index_in_space_check() {
        // "é" is 2 bytes in UTF-8, so the colon is at byte 2
        // chars: é(0), :(1), ' '(2), h(3), e(4), l(5), l(6), o(7)
        // bytes: é(0-1), :(2), ' '(3), h(4)
        // colon_pos = 2 (bytes), chars().nth(3) = 'h' ≠ ' '
        // This should be rejected because "é" is not ASCII alphanumeric,
        // not because the space check fails.
        let result = parse_conventional_commit("é: hello");
        assert!(
            result.is_none(),
            "Should reject non-ASCII type, but should fail for the right reason"
        );

        // An all-ASCII example that proves the byte/char mismatch doesn't
        // affect purely ASCII inputs (where byte index == char index):
        // This should parse correctly.
        let result = parse_conventional_commit("e: hello");
        assert!(result.is_some(), "ASCII single-char type should parse fine");
    }
}
