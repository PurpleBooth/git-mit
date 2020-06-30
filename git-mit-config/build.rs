use std::env;
use std::path::PathBuf;

use clap_generate::generators::{Bash, Elvish, Fish};

use mit_build_tools::completion;
use mit_build_tools::manpage;
use mit_commit_message_lints::lints::Lint;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let lint_names: Vec<_> = Lint::iterator()
        .map(mit_commit_message_lints::lints::Lint::name)
        .collect();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let app = cli::app(&lint_names);

    completion::generate::<Elvish>(&app, &out_dir.join("elvish_completion"));
    completion::generate::<Fish>(&app, &out_dir.join("fish_completion"));
    //completion::generate::<Zsh>(&app, &out_dir.join("zsh_completion")); // This segfaults at the moment
    completion::generate::<Bash>(&app, &out_dir.join("bash_completion"));

    manpage::generate(&app, &out_dir, "docs/manpage.template.md");
}
