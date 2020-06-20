use clap_generate::generators::{Bash, Elvish, Fish, Zsh};

use std::env;
use std::path::PathBuf;

extern crate tinytemplate;

#[path = "src/cli.rs"]
mod cli;
mod completion;
mod manpage;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let app = cli::app();

    completion::generate::<Elvish>(&app, &out_dir.join("elvish_completion"));
    completion::generate::<Fish>(&app, &out_dir.join("fish_completion"));
    completion::generate::<Zsh>(&app, &out_dir.join("zsh_completion"));
    completion::generate::<Bash>(&app, &out_dir.join("zsh_completion"));

    manpage::generate(&app, &out_dir);
}
