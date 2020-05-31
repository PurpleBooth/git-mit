use std::{env, fs::File, io::Write, process};

use clap::{crate_authors, crate_version, App, Arg};

use itertools::Itertools;

use pb_commit_message_lints::{
    author::{entities::Author, vcs::get_coauthor_configuration},
    errors::PbCommitMessageLintsError,
    external::vcs::Git2,
    lints::CommitMessage,
};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter},
    path::PathBuf,
};

fn display_err_and_exit<T>(error: &PbPrepareCommitMessageError) -> T {
    eprintln!("{}", error);
    process::exit(1);
}

fn main() {
    let matches = app().get_matches();

    let commit_message_path = matches
        .value_of("commit-message-path")
        .map(PathBuf::from)
        .expect("Expected commit file path");
    let current_dir = env::current_dir()
        .map_err(|err| PbPrepareCommitMessageError::new_io("$PWD".into(), &err))
        .unwrap_or_else(|err| display_err_and_exit(&err));

    let mut git_config = Git2::try_from(current_dir)
        .map_err(PbPrepareCommitMessageError::from)
        .unwrap_or_else(|err| display_err_and_exit(&err));

    if let Some(authors) = get_coauthor_configuration(&mut git_config)
        .map_err(PbPrepareCommitMessageError::from)
        .unwrap_or_else(|err| display_err_and_exit(&err))
    {
        append_coauthors_to_commit_message(commit_message_path, &authors)
            .map_err(PbPrepareCommitMessageError::from)
            .unwrap_or_else(|err| display_err_and_exit(&err))
    }
}

fn app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("commit-message-path")
                .help("The name of the file that contains the commit log message")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::with_name("commit-message-source")
                .help(
                    "The commit message, and can be: message (if a -m or -F option was given to \
                     git); template (if a -t option was given or the configuration option \
                     commit.template is set in git); merge (if the commit is a merge or a \
                     .git/MERGE_MSG file exists); squash (if a .git/SQUASH_MSG file exists); or \
                     commit",
                )
                .index(2)
                .required(false),
        )
        .arg(
            Arg::with_name("commit-sha")
                .help("Commit SHA-1 (if a -c, -C or --amend option was given to git).")
                .index(3)
                .required(false),
        )
}

fn append_coauthors_to_commit_message(
    commit_message_path: PathBuf,
    authors: &[Author],
) -> Result<(), PbPrepareCommitMessageError> {
    let path = String::from(commit_message_path.to_string_lossy());
    let commit_message = CommitMessage::try_from(commit_message_path.clone())?;
    File::create(commit_message_path)
        .and_then(|mut file| {
            file.write_all(
                format!(
                    r#"{}
{}
"#,
                    authors
                        .iter()
                        .map(|x| format!("Co-authored-by: {} <{}>", x.name(), x.email()))
                        .join("\n"),
                    commit_message
                )
                .as_bytes(),
            )
        })
        .map_err(|err| PbPrepareCommitMessageError::new_io(path, &err))
}

#[derive(Debug)]
enum PbPrepareCommitMessageError {
    PbCommitMessageLintsError(PbCommitMessageLintsError),
    Io(String, String),
}

impl Display for PbPrepareCommitMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PbPrepareCommitMessageError::PbCommitMessageLintsError(error) => write!(f, "{}", error),
            PbPrepareCommitMessageError::Io(file_source, error) => write!(
                f,
                "Failed to read author config from `{}`:\n{}",
                file_source, error
            ),
        }
    }
}

impl From<PbCommitMessageLintsError> for PbPrepareCommitMessageError {
    fn from(from: PbCommitMessageLintsError) -> Self {
        PbPrepareCommitMessageError::PbCommitMessageLintsError(from)
    }
}

impl Error for PbPrepareCommitMessageError {}

impl PbPrepareCommitMessageError {
    fn new_io(source: String, error: &std::io::Error) -> PbPrepareCommitMessageError {
        PbPrepareCommitMessageError::Io(source, format!("{}", error))
    }
}
