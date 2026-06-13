use std::path::PathBuf;

use clap::Parser;
use clap_complete::Shell;

#[derive(Parser, Clone, Eq, PartialEq)]
#[clap(author, version, about)]
#[clap(bin_name = "mit-commit-msg")]
pub struct Args {
    /// Path to a temporary file that contains the commit message written by the
    /// developer
    ///
    /// When omitted the hook falls back to `<gitdir>/COMMIT_EDITMSG`, which
    /// is useful when the hook is invoked via a hook manager like lefthook
    /// that does not forward git's positional argument.
    #[clap(index = 1)]
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
