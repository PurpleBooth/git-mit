use std::path::PathBuf;

use clap::Parser;
use clap_complete::Shell;
use mit_commit_message_lints::mit::lib::non_clean_behaviour::BehaviourOption;

#[derive(Parser, Clone, Eq, PartialEq)]
#[clap(author, version, about)]
#[clap(bin_name = "mit-prepare-commit-msg")]
pub struct Args {
    /// The name of the file that contains the commit log message
    #[clap(index = 1, required_unless_present = "completion")]
    pub commit_message_path: Option<PathBuf>,

    /// The commit message, and can be: message (if a -m or -F option was given
    /// to git); template (if a -t option was given or the configuration option
    /// commit.template is set in git); merge (if the commit is a merge or a
    /// `.git/MERGE_MSG` file exists); squash (if a `.git/SQUASH_MSG` file
    /// exists); or commit
    #[clap(index = 2)]
    pub commit_message_source: Option<PathBuf>,

    /// Commit SHA-1 (if a -c, -C or --amend option was given to git).
    #[clap(index = 3)]
    pub commit_sha: Option<String>,

    /// A command to execute to get the value for the "relates to" trailer
    #[clap(long, env = "GIT_MIT_RELATES_TO_EXEC")]
    pub relates_to_exec: Option<String>,
    /// A template to apply to the "relates to" trailer
    #[clap(long, env = "GIT_MIT_RELATES_TO_TEMPLATE")]
    pub relates_to_template: Option<String>,
    /// What to do when we rebase
    #[clap(long, env = "GIT_MIT_SET_NON_CLEAN_BEHAVIOUR")]
    pub non_clean_behaviour_option: Option<BehaviourOption>,
    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,
}
