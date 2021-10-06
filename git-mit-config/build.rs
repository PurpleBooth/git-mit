use std::{env, path::PathBuf};

use cli::app;
use mit_build_tools::manpage;
use mit_lint::Lint;

#[path = "src/cli/mod.rs"]
mod cli;

fn main() {
    let lint_names: Vec<_> = Lint::all_lints().map(mit_lint::Lint::name).collect();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let app = app::app(&lint_names);
    manpage::generate(&app, &out_dir, "docs/manpage.template.md");
}
