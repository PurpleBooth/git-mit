use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitMitConfigError {
    #[error("lint name not given")]
    LintNameNotGiven,
    #[error("author file not set")]
    AuthorFileNotSet,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unrecognised subcommand")]
#[diagnostic(
    code(git_mit::config::author::load),
    help("try `git mit-config --help`")
)]
pub struct UnrecognisedLintCommand {}

#[derive(Error, Debug, Diagnostic)]
pub enum LibGit2 {
    #[error("unable to discover git repository")]
    #[diagnostic(
        code(git_mit::config::author::load),
        help("is the directory a git repository")
    )]
    DiscoverGitRepository {
        #[source]
        source: git2::Error,
    },

    #[error("unable to read the configuration from the git repository")]
    #[diagnostic(
        code(git_mit::config::author::load),
        help("is there a problem with the git repository config?")
    )]
    ReadConfigFromGitRepository {
        #[source]
        source: git2::Error,
    },
    #[error("unable to read git's configuration")]
    #[diagnostic(
        code(git_mit::config::author::load),
        help("is there a problem with the git user config?")
    )]
    ReadUserConfigFromGit {
        #[source]
        source: git2::Error,
    },
}
