use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static> {
    App::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("completion")
                .short('c')
                .long("completion")
                .possible_values(&["bash", "elvish", "fish", "powershell", "zsh"]),
        )
}
