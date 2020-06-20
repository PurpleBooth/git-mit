use clap_generate::generators::{Bash, Elvish, Fish, Zsh};
use clap_generate::{generate_to, Generator};
use std::{env, fs};

use std::path::PathBuf;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    generate_completion::<Elvish>(&out_dir.join("elvish_completion"));
    generate_completion::<Fish>(&out_dir.join("fish_completion"));
    generate_completion::<Zsh>(&out_dir.join("zsh_completion"));
    generate_completion::<Bash>(&out_dir.join("zsh_completion"));
}

fn generate_completion<T>(dir: &PathBuf)
where
    T: Generator,
{
    if dir.exists() {
        fs::remove_dir_all(dir.clone()).unwrap();
    }

    fs::create_dir(dir.clone()).unwrap();
    generate_to::<T, _, _>(&mut cli::app(), env!("CARGO_PKG_NAME"), &dir);
}
