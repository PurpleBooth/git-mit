use std::path::PathBuf;

use clap::Parser;
use clap_complete::Shell;

#[derive(Parser, Clone, Eq, PartialEq)]
#[clap(author, version, about)]
#[clap(bin_name = "mit-commit-msg")]
pub struct Args {
    /// Path to a temporary file that contains the commit message written by the
    /// developer
    #[clap(index = 1, required_unless_present = "completion")]
    pub commit_file_path: Option<PathBuf>,

    /// On lint failure copy the message to clipboard
    #[clap(
        long,
        env = "GIT_MIT_COPY_MESSAGE_TO_CLIPBOARD",
        default_value = "true"
    )]
    pub copy_message_to_clipboard: bool,

    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,
}
