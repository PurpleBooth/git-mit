use std::{env, fs};

use git2::{Config, Repository};

use clap::{crate_authors, crate_version, App, Arg};
use pb_commit_message_lints::{
    author::{
        entities::{Author, Authors},
        vcs::set_authors,
        yaml::get_authors_from_user_config,
    },
    external::vcs::Git2,
};
use std::{error::Error, path::PathBuf, time::Duration};
use xdg::BaseDirectories;

const AUTHOR_INITIAL: &str = "author-initial";
const AUTHOR_FILE_PATH: &str = "author-file-path";

const TIMEOUT: &str = "timeout";

fn main() {
    let cargo_package_name = env!("CARGO_PKG_NAME");
    let default_config_file = config_file_path(cargo_package_name);

    let matches = App::new(cargo_package_name)
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

    let expires_in = matches
        .value_of(TIMEOUT)
        .ok_or_else(|| -> Box<dyn Error> { "No timeout set".into() })
        .and_then(|x| -> Result<u64, Box<dyn Error>> { x.parse::<u64>().map_err(Box::from) })
        .unwrap();
    let author_config_path = matches.value_of(AUTHOR_FILE_PATH).unwrap();
    let author_config =
        fs::read_to_string(author_config_path).expect("Something went wrong reading the file");

    let author_initial: Vec<&str> = matches.values_of(AUTHOR_INITIAL).unwrap().take(1).collect();
    let authors: Authors = get_authors_from_user_config(&author_config).unwrap();
    let author: Vec<Option<&Author>> = authors.get(&author_initial);

    let current_dir = env::current_dir().expect("Unable to retrieve current directory");
    let get_repository_config = |x: Repository| x.config();
    let get_default_config = |_| Config::open_default();
    let mut git_config = Repository::discover(current_dir)
        .and_then(get_repository_config)
        .or_else(get_default_config)
        .map(Git2::new)
        .expect("Couldn't load any git config");

    let authors = author.into_iter().flatten().collect::<Vec<_>>();
    set_authors(
        &mut git_config,
        &authors,
        Duration::from_secs(expires_in * 60),
    )
    .expect("Couldn't set author")
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
