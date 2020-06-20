use clap_generate::generators::{Bash, Elvish, Fish, Zsh};
use clap_generate::{generate, Generator};
use std::{env, fs};

use std::path::PathBuf;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let cargo_package_name = env!("CARGO_PKG_NAME");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    generate_completion::<Elvish>(
        format!("{}.elv", cargo_package_name),
        &out_dir.join("elvish_completion"),
    );
    generate_completion::<Fish>(
        format!("{}.fish", cargo_package_name),
        &out_dir.join("fish_completion"),
    );
    generate_completion::<Zsh>(
        format!("_{}", cargo_package_name),
        &out_dir.join("zsh_completion"),
    );
    generate_completion::<Bash>(cargo_package_name.into(), &out_dir.join("zsh_completion"));
}

fn generate_completion<T>(filename: String, dir: &PathBuf)
where
    T: Generator,
{
    if dir.exists() {
        fs::remove_dir_all(dir.clone()).unwrap();
    }

    fs::create_dir(dir.clone()).unwrap();
    let file_path = dir.join(filename);
    let mut file = fs::File::create(file_path).unwrap();
    generate::<T, _>(&mut cli::app(), env!("CARGO_PKG_NAME"), &mut file);
}
