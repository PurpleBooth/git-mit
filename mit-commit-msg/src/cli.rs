use clap::{crate_authors, crate_version, Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("commit-file-path")
                .help(
                    "Path to a temporary file that contains the commit message written by the \
                     developer",
                )
                .index(1)
                .required_unless_present("completion"),
        )
        .arg(
            Arg::new("copy-message-to-clipboard")
                .long("copy-message-to-clipboard")
                .help("On lint failure copy the message to clipboard")
                .env("GIT_MIT_COPY_MESSAGE_TO_CLIPBOARD")
                .takes_value(true)
                .default_value("true"),
        )
        .arg(Arg::new("completion").long("completion").possible_values(&[
            "bash",
            "elvish",
            "fish",
            "powershell",
            "zsh",
        ]))
}
