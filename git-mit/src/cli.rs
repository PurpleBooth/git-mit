use clap::{crate_authors, crate_version, App, Arg};

pub(crate) fn app(config_file_path: &str) -> App {
    App::new(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("initials")
                .about("Initials of the author to put in the commit")
                .multiple(true)
                .required_unless("completion")
                .min_values(1),
        )
        .arg(
            Arg::with_name("file")
                .short('c')
                .long("config")
                .about("Path to a file where author initials, emails and names can be found")
                .env("GIT_MIT_AUTHORS_CONFIG")
                .default_value(config_file_path),
        )
        .arg(
            Arg::with_name("command")
                .short('e')
                .long("exec")
                .about(
                    "Execute a command to generate the author configuration, stdout will be \
                 captured and used instead of the file, if both this and the file is present, \
                 this takes precedence",
                )
                .env("GIT_MIT_AUTHORS_EXEC"),
        )
        .arg(
            Arg::with_name("timeout")
                .short('t')
                .long("timeout")
                .about("Number of minutes to expire the configuration in")
                .env("GIT_MIT_AUTHORS_TIMEOUT")
                .default_value("60"),
        )
        .arg(
            Arg::with_name("completion")
                .long("completion")
                .about("Print completion information for your shell")
                .possible_values(&["bash", "fish", "zsh", "elvish"]),
        )
}
