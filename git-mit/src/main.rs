use std::{
    convert::TryFrom,
    env,
    error::Error,
    fmt::{Display, Formatter},
    fs, io,
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use clap::{crate_authors, crate_version, App, Arg, ArgMatches};
use xdg::BaseDirectories;

use mit_commit_message_lints::{
    author::{
        entities::{Author, Authors},
        vcs::set_authors,
    },
    errors::MitCommitMessageLintsError,
    external::vcs::Git2,
};

use crate::GitMitError::NoAuthorInitialsProvided;
use crate::{ExitCode::InitialNotMatchedToAuthor, GitMitError::NoTimeoutSet};
use clap_generate::generate;
use clap_generate::generators::{Bash, Elvish, Fish, Zsh};

#[repr(i32)]
enum ExitCode {
    InitialNotMatchedToAuthor = 3,
}

const AUTHOR_INITIAL: &str = "initials";
const AUTHOR_FILE_PATH: &str = "file";
const AUTHOR_FILE_COMMAND: &str = "command";
const TIMEOUT: &str = "timeout";
const COMPLETION: &str = "completion";

fn main() -> Result<(), GitMitError> {
    let path = config_path(env!("CARGO_PKG_NAME"))?;
    let matches = app(&path).get_matches();

    if let Some(shell) = matches.value_of(COMPLETION) {
        if shell == "bash" {
            generate::<Bash, _>(&mut app(&path), env!("CARGO_PKG_NAME"), &mut io::stdout())
        } else if shell == "fish" {
            generate::<Fish, _>(&mut app(&path), env!("CARGO_PKG_NAME"), &mut io::stdout())
        } else if shell == "zsh" {
            generate::<Zsh, _>(&mut app(&path), env!("CARGO_PKG_NAME"), &mut io::stdout())
        } else if shell == "elvish" {
            generate::<Elvish, _>(&mut app(&path), env!("CARGO_PKG_NAME"), &mut io::stdout())
        }

        return Ok(());
    }

    let users_config = get_users_config(&matches)?;
    let authors_initials = get_author_initials(&matches).ok_or_else(|| NoAuthorInitialsProvided)?;

    let all_authors = Authors::try_from(users_config.as_str())?;
    let selected_authors = all_authors.get(&authors_initials);
    let initials_without_authors = find_initials_missing(authors_initials, &selected_authors);

    if !initials_without_authors.is_empty() {
        exit_initial_not_matched_to_author(&initials_without_authors);
    }

    let current_dir =
        env::current_dir().map_err(|error| GitMitError::new_io("$PWD".into(), &error))?;

    let mut git_config = Git2::try_from(current_dir)?;

    let authors = selected_authors.into_iter().flatten().collect::<Vec<_>>();
    set_authors(
        &mut git_config,
        &authors,
        Duration::from_secs(get_timeout(&matches)? * 60),
    )?;

    Ok(())
}

fn exit_initial_not_matched_to_author(initials_without_authors: &[&str]) {
    eprintln!(
        r#"
Could not find the initials {}.

You can fix this by checking the initials are in the configuration file.
"#,
        initials_without_authors.join(", "),
    );

    std::process::exit(InitialNotMatchedToAuthor as i32);
}

fn find_initials_missing<'a>(
    authors_initials: Vec<&'a str>,
    selected_authors: &[Option<&Author>],
) -> Vec<&'a str> {
    selected_authors
        .iter()
        .zip(authors_initials)
        .filter_map(|(result, initial)| match result {
            None => Some(initial),
            Some(_) => None,
        })
        .collect()
}

fn app(config_file_path: &str) -> App {
    App::new(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(AUTHOR_INITIAL)
                .about("Initials of the author to put in the commit")
                .multiple(true)
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name(AUTHOR_FILE_PATH)
                .short('c')
                .long("config")
                .about("Path to a file where author initials, emails and names can be found")
                .env("GIT_MIT_AUTHORS_CONFIG")
                .default_value(config_file_path),
        )
        .arg(
            Arg::with_name(AUTHOR_FILE_COMMAND)
                .short('e')
                .long("exec")
                .about(
                    "Execute a command to generate the author configuration, stdout will be \
                     captured and used instead of the file, if both this and the file is present, \
                     this takes precedence",
                )
                .env("GIT_MIT_AUTHORS_EXEC"),
        )
        .arg(
            Arg::with_name(TIMEOUT)
                .short('t')
                .long("timeout")
                .about("Number of minutes to expire the configuration in")
                .env("GIT_MIT_AUTHORS_TIMEOUT")
                .default_value("60"),
        )
        .arg(
            Arg::with_name(COMPLETION)
                .long("completion")
                .about("Print completion information for your shell")
                .possible_values(&["bash", "fish", "zsh", "elvish"]),
        )
}

fn get_author_initials<'a>(matches: &'a ArgMatches) -> Option<Vec<&'a str>> {
    matches.values_of(AUTHOR_INITIAL).map(Iterator::collect)
}

fn get_users_config(matches: &ArgMatches) -> Result<String, GitMitError> {
    match matches.value_of(AUTHOR_FILE_COMMAND) {
        Some(command) => get_author_config_from_exec(command),
        None => get_author_config_from_file(matches),
    }
}

fn get_author_config_from_exec(command: &str) -> Result<String, GitMitError> {
    Command::new(env::var("SHELL").unwrap_or_else(|_| String::from("sh")))
        .stderr(Stdio::inherit())
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|error| GitMitError::new_io(command.into(), &error))
        .and_then(|x| String::from_utf8(x.stdout).map_err(GitMitError::from))
}

fn get_author_config_from_file(matches: &ArgMatches) -> Result<String, GitMitError> {
    get_author_file_path(&matches)
        .ok_or_else(|| GitMitError::AuthorFileNotSet)
        .and_then(|path| {
            fs::read_to_string(path).map_err(|error| GitMitError::new_io(path.into(), &error))
        })
}

fn get_author_file_path(matches: &ArgMatches) -> Option<&str> {
    matches.value_of(AUTHOR_FILE_PATH)
}

fn get_timeout(matches: &ArgMatches) -> Result<u64, GitMitError> {
    matches
        .value_of(TIMEOUT)
        .ok_or_else(|| NoTimeoutSet)
        .and_then(|x| x.parse().map_err(GitMitError::from))
}

fn config_path(cargo_package_name: &str) -> Result<String, GitMitError> {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(GitMitError::from)
        .and_then(|base| authors_config_file(&base))
        .map(|path| path.to_string_lossy().into())
}

fn authors_config_file(config_directory: &BaseDirectories) -> Result<PathBuf, GitMitError> {
    config_directory
        .place_config_file("mit.yml")
        .map_err(|error| GitMitError::new_io("<config_dir>/author.yml".into(), &error))
}

#[derive(Debug)]
enum GitMitError {
    NoAuthorInitialsProvided,
    NoTimeoutSet,
    PbCommitMessageLints(MitCommitMessageLintsError),
    Io(String, String),
    Xdg(String),
    TimeoutNotNumber(String),
    Utf8(String),
    AuthorFileNotSet,
}

impl GitMitError {
    fn new_io(source: String, error: &std::io::Error) -> GitMitError {
        GitMitError::Io(source, format!("{}", error))
    }
}

impl Display for GitMitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GitMitError::NoAuthorInitialsProvided => write!(f, "No author initials provided"),
            GitMitError::NoTimeoutSet => write!(f, "No timeout set"),
            GitMitError::TimeoutNotNumber(error) => write!(
                f,
                "The timeout needs to be the number of minutes:\n{}",
                error
            ),
            GitMitError::PbCommitMessageLints(error) => write!(f, "{}", error),
            GitMitError::Io(file_source, error) => {
                write!(f, "Failed to read from `{}`:\n{}", file_source, error)
            }
            GitMitError::Xdg(error) => write!(f, "Failed to find config directory: {}", error),
            GitMitError::AuthorFileNotSet => {
                write!(f, "Expected a author file path, didn't find one")
            }
            GitMitError::Utf8(error) => write!(
                f,
                "Failed to convert the output from the author file generation command to a UTF-8 \
                 String:\n{}",
                error
            ),
        }
    }
}

impl From<std::string::FromUtf8Error> for GitMitError {
    fn from(from: std::string::FromUtf8Error) -> Self {
        GitMitError::Utf8(format!("{}", from))
    }
}

impl From<MitCommitMessageLintsError> for GitMitError {
    fn from(from: MitCommitMessageLintsError) -> Self {
        GitMitError::PbCommitMessageLints(from)
    }
}

impl From<std::num::ParseIntError> for GitMitError {
    fn from(from: std::num::ParseIntError) -> Self {
        GitMitError::TimeoutNotNumber(format!("{}", from))
    }
}

impl From<xdg::BaseDirectoriesError> for GitMitError {
    fn from(from: xdg::BaseDirectoriesError) -> Self {
        GitMitError::Xdg(format!("{}", from))
    }
}

impl Error for GitMitError {}
