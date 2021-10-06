use std::{env, path::PathBuf};

use mit_build_tools::manpage;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let app = cli::app();
    manpage::generate(&app, &out_dir, "docs/manpage.template.md");
}
