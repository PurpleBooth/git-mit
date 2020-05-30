use std::{env, process};

use clap::{crate_authors, crate_version, App};

use pb_commit_message_lints::{author::vcs::get_coauthor_configuration, external::vcs::Git2};
use std::convert::TryFrom;

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

    let current_dir = env::current_dir().unwrap();

    let mut git_config = Git2::try_from(current_dir).unwrap();

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
