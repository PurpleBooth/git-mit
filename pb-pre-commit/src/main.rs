use std::env;
use std::os::unix::process::CommandExt;
use std::process;

use clap::{crate_authors, crate_version, App};

fn main() {
    App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let cmd = "git";
    let err = process::Command::new(cmd).exec();
    panic!("panic!: {}", err)
}
