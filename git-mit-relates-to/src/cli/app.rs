use clap::Parser;
use clap_complete::Shell;

#[derive(Parser, Clone, Eq, PartialEq)]
#[clap(author, version, about)]
#[clap(bin_name = "git-mit-relates-to")]
pub struct Args {
    /// The issue number or other string to place into the Relates-to trailer
    #[clap(required_unless_present = "completion")]
    pub issue_number: Option<String>,
    /// Number of minutes to expire the configuration in
    #[clap(long, short, env = "GIT_MIT_RELATES_TO_TIMEOUT", default_value = "60")]
    pub timeout: u64,

    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,
}
