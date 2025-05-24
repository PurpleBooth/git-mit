use clap::Parser;
use clap_complete::Shell;
use indoc::indoc;
use mit_commit_message_lints::mit::AuthorArgs;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(bin_name = "git-mit")]
#[clap( after_help = indoc!(
    "
    COMMON TASKS:
        You can install git-mit into a new repository using

            git mit-install

        You can add a new author to that repository by running

            git mit-config mit set eg \"Egg Sample\" egg.sample@example.com

        You can save that author permanently by running

            git mit-config mit set eg \"Egg Sample\" egg.sample@example.com
            git mit-config mit generate > $HOME/.config/git-mit/mit.toml

        You can disable a lint by running

            git mit-config lint disable jira-issue-key-missing

        You can install the example authors file to the default location with

            git mit-config mit example > $HOME/.config/git-mit/mit.toml

        You can set the current author, and Co-authors by running

            git mit ae se

        You can populate the `Relates-to` trailer using

            git mit-relates-to \"[#12345678]\"
    "
))]
pub struct CliArgs {
    /// Initials of the mit to put in the commit
    #[clap(required_unless_present = "completion")]
    pub initials: Vec<String>,

    /// Path to a file where mit initials, emails and names can be found
    #[clap(
        short,
        long,
        env = "GIT_MIT_AUTHORS_CONFIG",
        default_value = "$HOME/.config/git-mit/mit.toml"
    )]
    pub config: String,

    /// Execute a command to generate the mit configuration,
    /// stdout will be captured and used instead of the file,
    /// if both this and the file are present, this takes precedence
    #[clap(short, long, env = "GIT_MIT_AUTHORS_EXEC")]
    pub exec: Option<String>,

    /// Number of minutes to expire the configuration in
    #[clap(short, long, env = "GIT_MIT_AUTHORS_TIMEOUT", default_value = "60")]
    pub timeout: u64,

    /// Shell to generate completions for
    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,
}

impl AuthorArgs for CliArgs {
    fn author_command(&self) -> Option<&str> {
        self.exec.as_deref()
    }

    fn author_file(&self) -> Option<&str> {
        Some(&self.config)
    }
}
