use std::{env, process};

use clap::{crate_authors, crate_version, App};
use git2::{Config, Repository};

use pb_commit_author::get_author_configuration;

#[repr(i32)]
enum ExitCode {
    StaleAuthor = 1,
}

fn main() {
    App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let current_dir = env::current_dir().expect("Unable to retrieve current directory");

    let get_config_from_repository = |x: Repository| x.config();
    let get_default_config = |_| Config::open_default();

    let git_config = Repository::discover(current_dir)
        .and_then(get_config_from_repository)
        .or_else(get_default_config)
        .expect("Couldn't load any git config")
        .snapshot()
        .expect("Could not freeze git config");

    if let None = get_author_configuration(&git_config) {
        eprintln!(
            r#"
The details of the author of this commit are a bit stale. Can you confirm who's currently coding?

It's nice to get and give the right credit.

You can fix this by running `git author` then the initials of whoever is coding for example:
git author bt
git author bt se"#,
        );

        process::exit(ExitCode::StaleAuthor as i32);
    }
}
