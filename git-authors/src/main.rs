use std::env;

use clap::{crate_authors, crate_version, App, Arg};
use std::{error::Error, path::PathBuf};
use xdg::BaseDirectories;

const AUTHOR_INITIAL: &str = "author-initial";
const AUTHOR_FILE_PATH: &str = "author-file-path";

const TIMEOUT: &str = "timeout";

fn main() {
    let cargo_package_name = env!("CARGO_PKG_NAME");
    let default_config_file = config_file_path(cargo_package_name);

    App::new(cargo_package_name)
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name(AUTHOR_INITIAL)
                .help("Initials of the authors to put in the commit")
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::with_name(AUTHOR_FILE_PATH)
                .short("c")
                .long("config")
                .help("Initials of the authors to put in the commit")
                .env("GIT_AUTHORS_AUTHOR_FILE_PATH")
                .default_value(&default_config_file),
        )
        .arg(
            Arg::with_name(TIMEOUT)
                .short("t")
                .long("timeout")
                .help("Number of minutes to expire the configuration in")
                .env("GIT_AUTHORS_TIMEOUT")
                .default_value("60"),
        )
        .get_matches();
}

fn config_file_path(cargo_package_name: &str) -> String {
    xdg::BaseDirectories::with_prefix(cargo_package_name.to_string())
        .map_err(Box::<dyn std::error::Error>::from)
        .and_then(|x| authors_config_file(&x))
        .unwrap()
        .to_string_lossy()
        .to_string()
}

fn authors_config_file(x: &BaseDirectories) -> Result<PathBuf, Box<dyn Error>> {
    x.place_config_file("authors.yml").map_err(Box::from)
}
