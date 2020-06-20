use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("commit-file-path")
                .about(
                    "Path to a temporary file that contains the commit message written by the \
                 developer",
                )
                .index(1)
                .required(true),
        )
}
