use clap::Parser;
use clap_complete::Shell;

#[derive(Parser, Clone, Eq, PartialEq)]
#[clap(author, version, about)]
#[clap(bin_name = "mit-pre-commit")]
pub struct Args {
    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,
}
