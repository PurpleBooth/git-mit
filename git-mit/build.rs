use clap_generate::generators::{Bash, Elvish, Fish, Zsh};
use clap_generate::{generate, Generator};
use std::{env, fs};

use std::path::PathBuf;

#[path = "src/cli.rs"]
mod cli;

fn main() {
    let cargo_package_name = env!("CARGO_PKG_NAME");
    let base = xdg::BaseDirectories::with_prefix(cargo_package_name.to_string()).unwrap();
    let config_file = base.place_config_file("mit.yml").unwrap();
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let config_path_str = config_file.to_str().unwrap();

    generate_completion::<Elvish>(
        &config_path_str,
        format!("{}.elv", cargo_package_name),
        &out_dir.join("elvish_completion"),
    );
    generate_completion::<Fish>(
        &config_path_str,
        format!("{}.fish", cargo_package_name),
        &out_dir.join("fish_completion"),
    );
    generate_completion::<Zsh>(
        &config_path_str,
        format!("_{}", cargo_package_name),
        &out_dir.join("zsh_completion"),
    );
    generate_completion::<Bash>(
        &config_path_str,
        cargo_package_name.into(),
        &out_dir.join("zsh_completion"),
    );
}

fn generate_completion<T>(config_path_str: &&str, filename: String, dir: &PathBuf)
where
    T: Generator,
{
    if dir.exists() {
        fs::remove_dir_all(dir.clone()).unwrap();
    }

    fs::create_dir(dir.clone()).unwrap();
    let file_path = dir.join(filename);
    let mut file = fs::File::create(file_path).unwrap();
    generate::<T, _>(
        &mut cli::app(&config_path_str),
        env!("CARGO_PKG_NAME"),
        &mut file,
    );
}
