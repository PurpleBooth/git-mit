use clap::{crate_authors, crate_version, App, AppSettings, Arg};

pub fn app<'a>(lint_names: &'a [&str]) -> App<'a> {
    let lint_argument = Arg::with_name("lint")
        .about("The lint to enable")
        .required(true)
        .multiple(true)
        .min_values(1)
        .possible_values(lint_names);
    App::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("scope")
                .long("scope")
                .short('s')
                .possible_values(&["local", "global"])
                .default_value("local"),
        )
        .subcommand(
            App::new("lint")
                .about("Manage active lints")
                .subcommand(App::new("available").about("List the available lints"))
                .subcommand(App::new("enabled").about("List the enabled lints"))
                .subcommand(
                    App::new("status")
                        .about("Get status of a lint")
                        .arg(lint_argument.clone()),
                )
                .subcommand(
                    App::new("enable")
                        .about("Enable a lint")
                        .arg(lint_argument.clone()),
                )
                .subcommand(
                    App::new("disable")
                        .about("Disable a lint")
                        .arg(lint_argument.clone()),
                )
                .setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .subcommand(
            App::new("mit")
                .about("Manage author configuration")
                .subcommand(App::new("example").about("Print example author yaml file"))
                .setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}
