use clap_generate::generators::{Bash, Elvish, Fish, Zsh};

use std::env;
use std::path::PathBuf;

#[path = "src/cli.rs"]
mod cli;
use mit_build_tools::completion;
use mit_build_tools::manpage;

fn main() {
    let cargo_package_name = env!("CARGO_PKG_NAME");
    let base = xdg::BaseDirectories::with_prefix(cargo_package_name.to_string()).unwrap();
    let config_file = base.place_config_file("mit.yml").unwrap();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let config_path_str = config_file.to_str().unwrap();

    let app = cli::app(&config_path_str);

    completion::generate::<Elvish>(&app, &out_dir.join("elvish_completion"));
    completion::generate::<Fish>(&app, &out_dir.join("fish_completion"));
    completion::generate::<Zsh>(&app, &out_dir.join("zsh_completion"));
    completion::generate::<Bash>(&app, &out_dir.join("bash_completion"));

    manpage::generate(&app, &out_dir, "docs/manpage.template.md");
}
