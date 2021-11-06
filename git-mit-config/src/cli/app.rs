use clap::{crate_authors, crate_version, App, AppSettings, Arg};

use super::super::cmd::author_example;
use crate::cmd::{
    author_generate,
    author_set,
    lint_available,
    lint_disable,
    lint_enable,
    lint_enabled,
    lint_generate,
    lint_status,
    relates_to_template,
};

#[allow(clippy::too_many_lines)]
pub fn app<'a>(lint_names: &'a [&str]) -> App<'a> {
    App::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            App::new("lint")
                .about("Manage active lints")
                .subcommand(lint_generate::app())
                .subcommand(lint_available::app())
                .subcommand(lint_enabled::app())
                .subcommand(lint_status::app(lint_names))
                .subcommand(lint_enable::app(lint_names))
                .subcommand(lint_disable::app(lint_names))
                .setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .subcommand(
            App::new("mit")
                .about("Manage mit configuration")
                .subcommand(author_set::app())
                .subcommand(author_generate::app_generate())
                .subcommand(author_generate::app_available())
                .subcommand(author_example::app())
                .setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .subcommand(
            App::new("relates-to")
                .about("Manage relates-to settings")
                .subcommand(relates_to_template::app())
                .setting(AppSettings::SubcommandRequiredElseHelp),
        )
        .arg(Arg::new("completion").long("completion").possible_values(&[
            "bash",
            "elvish",
            "fish",
            "powershell",
            "zsh",
        ]))
}
