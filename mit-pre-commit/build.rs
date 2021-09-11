use std::{env, path::PathBuf};

use clap_generate::generators::{Bash, Elvish, Fish, PowerShell, Zsh};
use mit_build_tools::{completion, manpage};

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let app = cli::app();

    completion::generate::<Elvish>(&app, &out_dir.join("elvish_completion"));
    completion::generate::<Fish>(&app, &out_dir.join("fish_completion"));
    completion::generate::<Zsh>(&app, &out_dir.join("zsh_completion"));
    completion::generate::<Bash>(&app, &out_dir.join("bash_completion"));
    completion::generate::<PowerShell>(&app, &out_dir.join("power_shell_completion"));

    manpage::generate(&app, &out_dir, "docs/manpage.template.md");
}
