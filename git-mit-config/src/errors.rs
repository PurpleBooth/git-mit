use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum GitMitConfigError {
    #[error("lint name not given")]
    #[diagnostic(
        url(docsrs),
        code(git_mit_config::errors::git_mit_config_error::lint_name_not_given)
    )]
    LintNameNotGiven,
    #[error("author file not set")]
    #[diagnostic(
        url(docsrs),
        code(git_mit_config::errors::git_mit_config_error::author_file_not_set)
    )]
    AuthorFileNotSet,
}

#[derive(Error, Debug, Diagnostic)]
#[error("unrecognised subcommand")]
#[diagnostic(
    code(git_mit_config::errors::unrecognised_lint_command),
    url(docsrs),
    help("try `git mit-config --help`")
)]
pub struct UnrecognisedLintCommand {}

#[derive(Error, Debug, Diagnostic)]
pub enum LibGit2 {
    #[error("unable to discover git repository")]
    #[diagnostic(
        code(git_mit_config::errors::lib_git2::discover_git_repository),
        url(docsrs),
        help("is the directory a git repository")
    )]
    DiscoverGitRepository {
        #[source]
        source: git2::Error,
    },

    #[error("unable to read the configuration from the git repository")]
    #[diagnostic(
        code(git_mit_config::errors::lib_git2::read_config_from_git_repository),
        url(docsrs),
        help("is there a problem with the git repository config?")
    )]
    ReadConfigFromGitRepository {
        #[source]
        source: git2::Error,
    },
    #[error("unable to read git's configuration")]
    #[diagnostic(
        code(git_mit_config::errors::lib_git2::read_user_config_from_git),
        url(docsrs),
        help("is there a problem with the git user config?")
    )]
    ReadUserConfigFromGit {
        #[source]
        source: git2::Error,
    },
}
