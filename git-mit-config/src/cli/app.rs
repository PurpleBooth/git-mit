use clap::{crate_authors, crate_version, Arg, Command};

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
pub fn cli<'a>(lint_names: &'a [&str]) -> Command<'a> {
    Command::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            Command::new("lint")
                .about("Manage active lints")
                .subcommand(lint_generate::cli())
                .subcommand(lint_available::cli())
                .subcommand(lint_enabled::cli())
                .subcommand(lint_status::cli(lint_names))
                .subcommand(lint_enable::cli(lint_names))
                .subcommand(lint_disable::cli(lint_names))
                .subcommand_required(true)
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("mit")
                .about("Manage mit configuration")
                .subcommand(author_set::cli())
                .subcommand(author_generate::cli_generate())
                .subcommand(author_generate::cli_available())
                .subcommand(author_example::app())
                .subcommand_required(true)
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("relates-to")
                .about("Manage relates-to settings")
                .subcommand(relates_to_template::cli())
                .subcommand_required(true)
                .arg_required_else_help(true),
        )
        .arg(Arg::new("completion").long("completion").possible_values([
            "bash",
            "elvish",
            "fish",
            "powershell",
            "zsh",
        ]))
}
