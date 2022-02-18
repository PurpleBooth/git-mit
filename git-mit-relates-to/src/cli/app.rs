use clap::{crate_authors, crate_version, Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new(String::from(env!("CARGO_PKG_NAME")))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("issue-number")
                .help("The issue number or other string to place into the Relates-to trailer")
                .required_unless_present("completion"),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .help("Number of minutes to expire the configuration in")
                .env("GIT_MIT_RELATES_TO_TIMEOUT")
                .default_value("60")
                .takes_value(true),
        )
        .arg(Arg::new("completion").long("completion").possible_values(&[
            "bash",
            "elvish",
            "fish",
            "powershell",
            "zsh",
        ]))
}
