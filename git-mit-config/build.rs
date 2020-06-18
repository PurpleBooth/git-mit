use clap_generate::generate;
use clap_generate::generators::{Bash, Elvish, Fish};
use std::{env, fs};

use mit_commit_message_lints::lints::Lint;
use std::path::PathBuf;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let cargo_package_name = env!("CARGO_PKG_NAME");
    let lint_names: Vec<_> = Lint::iterator()
        .map(mit_commit_message_lints::lints::Lint::name)
        .collect();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let dir = out_dir.join("bash_completion");
    if dir.exists() {
        fs::remove_dir_all(dir.clone()).unwrap();
    }

    fs::create_dir(dir.clone()).unwrap();
    let file_path = dir.join(cargo_package_name.to_string());
    let mut file = fs::File::create(file_path).unwrap();
    generate::<Bash, _>(
        &mut cli::app(&lint_names),
        env!("CARGO_PKG_NAME"),
        &mut file,
    );

    let dir = out_dir.join("zsh_completion");
    if dir.exists() {
        fs::remove_dir_all(dir).unwrap();
    }

    let dir = out_dir.join("fish_completion");
    if dir.exists() {
        fs::remove_dir_all(dir.clone()).unwrap();
    }

    fs::create_dir(dir.clone()).unwrap();
    let file_path = dir.join(format!("{}.fish", cargo_package_name));
    let mut file = fs::File::create(file_path).unwrap();
    generate::<Fish, _>(
        &mut cli::app(&lint_names),
        env!("CARGO_PKG_NAME"),
        &mut file,
    );

    let dir = out_dir.join("elvish_completion");
    if dir.exists() {
        fs::remove_dir_all(dir.clone()).unwrap();
    }

    fs::create_dir(dir.clone()).unwrap();
    let file_path = dir.join(format!("{}.elv", cargo_package_name));
    let mut file = fs::File::create(file_path).unwrap();
    generate::<Elvish, _>(
        &mut cli::app(&lint_names),
        env!("CARGO_PKG_NAME"),
        &mut file,
    );
}
