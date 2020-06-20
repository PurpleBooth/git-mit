use clap_generate::generators::{Bash, Elvish, Fish};
use clap_generate::{generate_to, Generator};
use std::{env, fs};

use mit_commit_message_lints::lints::Lint;
use std::path::PathBuf;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let lint_names: Vec<_> = Lint::iterator()
        .map(mit_commit_message_lints::lints::Lint::name)
        .collect();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    generate_completion::<Elvish>(&lint_names, &out_dir.join("elvish_completion"));
    generate_completion::<Fish>(&lint_names, &out_dir.join("fish_completion"));
    // This segfaults at the moment
    // generate_completion::<Zsh>(&lint_names, &out_dir.join("zsh_completion"));
    generate_completion::<Bash>(&lint_names, &out_dir.join("bash_completion"));
}

fn generate_completion<T>(lint_names: &[&str], dir: &PathBuf)
where
    T: Generator,
{
    if dir.exists() {
        fs::remove_dir_all(dir.clone()).unwrap();
    }

    fs::create_dir(dir.clone()).unwrap();
    generate_to::<T, _, _>(&mut cli::app(lint_names), env!("CARGO_PKG_NAME"), &dir);
}
