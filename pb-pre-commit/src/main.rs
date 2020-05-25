use std::{env, process};

use clap::{crate_authors, crate_version, App};
use git2::{Config, Repository};

use pb_commit_message_lints::{author::vcs::get_coauthor_configuration, external::vcs::Git2};

#[repr(i32)]
enum ExitCode {
    StaleAuthor = 3,
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

    let mut git_config = Repository::discover(current_dir)
        .and_then(get_config_from_repository)
        .or_else(get_default_config)
        .map(Git2::new)
        .expect("Could not freeze git config");

    if get_coauthor_configuration(&mut git_config).is_none() {
        eprintln!(
            r#"
The details of the author of this commit are a bit stale. Can you confirm who's currently coding?

It's nice to get and give the right credit.

You can fix this by running `git authors` then the initials of whoever is coding for example:
git authors bt
git authors bt se"#,
        );

        process::exit(ExitCode::StaleAuthor as i32);
    }
}
