use clap::{crate_authors, crate_version, Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::new("completion").long("completion").possible_values(&[
            "bash",
            "elvish",
            "fish",
            "powershell",
            "zsh",
        ]))
}
