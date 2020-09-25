use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("commit-file-path")
                .about(
                    "Path to a temporary file that contains the commit message written by the \
                 developer",
                )
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("copy-message-to-clipboard")
                .long("copy-message-to-clipboard")
                .about("On lint failure copy the message to clipboard")
                .env("GIT_MIT_COPY_MESSAGE_TO_CLIPBOARD")
                .default_value("true"),
        )
}
