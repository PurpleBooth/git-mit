extern crate pb_commit_message_lints;

use std::{
    env,
    fmt,
    fmt::{Display, Formatter},
    fs,
};

use clap::{crate_authors, crate_version, App, Arg};
use git2::{Config, Repository};

use pb_commit_message_lints::{
    external::vcs::Git2,
    lints::{
        get_lint_configuration,
        has_duplicated_trailers,
        has_missing_jira_issue_key,
        has_missing_pivotal_tracker_id,
        CommitMessage,
        Lints,
    },
};

use crate::ExitCode::JiraIssueKeyMissing;

#[repr(i32)]
enum ExitCode {
    DuplicatedTrailers = 3,
    PivotalTrackerIdMissing,
    JiraIssueKeyMissing,
}

const COMMIT_FILE_PATH_NAME: &str = "commit-file-path";
const FIELD_SINGULAR: &str = "field";
const FIELD_PLURAL: &str = "fields";

fn main() -> std::io::Result<()> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(COMMIT_FILE_PATH_NAME)
                .help(
                    "Path to a temporary file that contains the commit message written by the \
                     developer",
                )
                .index(1)
                .required(true),
        )
        .get_matches();

    let commit_file_path = matches.value_of(COMMIT_FILE_PATH_NAME).unwrap();
    let commit_message =
        fs::read_to_string(commit_file_path).expect("Something went wrong reading the file");

    let current_dir = env::current_dir().expect("Unable to retrieve current directory");

    let get_repository_config = |x: Repository| x.config();
    let get_default_config = |_| Config::open_default();
    let git_config = Repository::discover(current_dir)
        .and_then(get_repository_config)
        .or_else(get_default_config)
        .map(Git2::new)
        .expect("Couldn't load any git config");

    get_lint_configuration(&git_config)
        .into_iter()
        .map(|x| match x {
            Lints::DuplicatedTrailers => {
                lint_duplicated_trailers(&commit_message);
                true
            },
            Lints::PivotalTrackerIdMissing => {
                lint_missing_pivotal_tracker_id(&commit_message);
                true
            },
            Lints::JiraIssueKeyMissing => {
                lint_missing_jira_issue_key(&commit_message);
                true
            },
        })
        .fold(Ok(()), |x, _| x)
}

struct LintProblem {
    help: String,
    code: ExitCode,
}

impl LintProblem {
    pub fn new(help: String, code: ExitCode) -> LintProblem {
        LintProblem { help, code }
    }

    pub fn code(self) -> ExitCode {
        self.code
    }
}

impl Display for LintProblem {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.help)
    }
}

fn lint_missing_jira_issue_key(commit_message: &str) -> Option<()> {
    let result = if has_missing_jira_issue_key(&CommitMessage::new(commit_message)) {
        Some(LintProblem::new(
            JIRA_HELP_MESSAGE.into(),
            JiraIssueKeyMissing,
        ))
    } else {
        None
    };

    result.map(|problem| exit(commit_message, problem))
}

fn lint_missing_pivotal_tracker_id(commit_message: &str) -> Option<()> {
    let result = if has_missing_pivotal_tracker_id(&CommitMessage::new(commit_message)) {
        Some(LintProblem::new(
            PIVOTAL_TRACKER_HELP.into(),
            ExitCode::PivotalTrackerIdMissing,
        ))
    } else {
        None
    };

    result.map(|problem| exit(commit_message, problem))
}

fn lint_duplicated_trailers(commit_message: &str) -> Option<()> {
    let duplicated_trailers = has_duplicated_trailers(&CommitMessage::new(commit_message));
    let result = if duplicated_trailers.is_empty() {
        None
    } else {
        let mut fields = FIELD_SINGULAR;
        if duplicated_trailers.len() > 1 {
            fields = FIELD_PLURAL
        }

        Some(LintProblem::new(
            format!(
                r#"
Your commit cannot have the same name duplicated in the "{}" {}

You can fix this by removing the duplicated field when you commit again
"#,
                duplicated_trailers.join("\", \""),
                fields
            ),
            ExitCode::DuplicatedTrailers,
        ))
    };

    result.map(|problem| exit(commit_message, problem))
}

const PIVOTAL_TRACKER_HELP: &str = r#"
Your commit is missing a Pivotal Tracker Id

You can fix this by adding the Id in one of the styles below to the commit message
[Delivers #12345678]
[fixes #12345678]
[finishes #12345678]
[#12345884 #12345678]
[#12345884,#12345678]
[#12345678],[#12345884]
This will address [#12345884]
"#;

const JIRA_HELP_MESSAGE: &str = r#"
Your commit is missing a JIRA Issue Key

You can fix this by adding a key like `JRA-123` to the commit message
"#;

fn exit(commit_message: &str, x: LintProblem) {
    eprintln!("\n{}\n{}", commit_message, x);

    std::process::exit(x.code() as i32);
}
