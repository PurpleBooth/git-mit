use std::path::PathBuf;

use clap::Parser;
use clap_complete::Shell;
use mit_commit_message_lints::scope::Scope;

#[cfg(not(target_os = "windows"))]
const HOME_ENV_KEY: &str = "HOME";

#[cfg(target_os = "windows")]
const HOME_ENV_KEY: &str = "USERPROFILE";

#[derive(Parser, Clone, Eq, PartialEq)]
#[clap(author, version, about)]
#[clap(bin_name = "git-mit-install")]
pub struct CliArgs {
    #[clap(short, long, default_value = "local", value_enum)]
    pub scope: Scope,

    #[clap(long, value_enum, value_parser)]
    pub completion: Option<Shell>,

    #[clap(long, env = HOME_ENV_KEY, required_if_eq("scope", "global"))]
    pub home_dir: Option<PathBuf>,
}
