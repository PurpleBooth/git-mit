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
                .subcommand(
                    App::new("generate")
                        .about("Generate the config file for your current settings"),
                )
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
                .about("Manage mit configuration")
                .subcommand(
                    App::new("set")
                        .arg(
                            Arg::with_name("initial")
                                .about("Initial of the mit to update or add")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("name")
                                .about("Name to use for the mit in format \"Forename Surname\"")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("email")
                                .about("Email to use for the mit")
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("signingkey")
                                .about("Signing key to use for this user")
                                .required(false),
                        )
                        .about("Update or add an initial in the mit configuration"),
                )
                .subcommand(
                    App::new("generate")
                        .arg(
                            Arg::with_name("file")
                                .short('c')
                                .long("config")
                                .about("Path to a file where mit initials, emails and names can be found")
                                .env("GIT_MIT_AUTHORS_CONFIG")
                                .default_value("$HOME/.config/git-mit/mit.yml"),
                        )
                        .arg(
                            Arg::with_name("command")
                                .short('e')
                                .long("exec")
                                .about(
                                    "Execute a command to generate the mit configuration, stdout will be \
                 captured and used instead of the file, if both this and the file is present, \
                 this takes precedence",
                                )
                                .env("GIT_MIT_AUTHORS_EXEC"),
                        )
                        .about("Generate a file version of available authors"),
                )
                .subcommand(App::new("example").about("Print example mit yaml file"))
                .setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
}
