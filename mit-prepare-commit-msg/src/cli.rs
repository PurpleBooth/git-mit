use clap::{crate_authors, crate_version, Arg, Command};

pub fn cli() -> Command<'static> {
    Command::new(env!("CARGO_PKG_NAME"))
        .bin_name(String::from(env!("CARGO_PKG_NAME")))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("commit-message-path")
                .help("The name of the file that contains the commit log message")
                .index(1)
                .required_unless_present("completion"),
        )
        .arg(
            Arg::new("commit-message-source")
                .help(
                    "The commit message, and can be: message (if a -m or -F option was given to \
                     git); template (if a -t option was given or the configuration option \
                     commit.template is set in git); merge (if the commit is a merge or a \
                     .git/MERGE_MSG file exists); squash (if a .git/SQUASH_MSG file exists); or \
                     commit",
                )
                .index(2)
                .required(false),
        )
        .arg(
            Arg::new("commit-sha")
                .help("Commit SHA-1 (if a -c, -C or --amend option was given to git).")
                .index(3)
                .required(false),
        )
        .arg(
            Arg::new("relates-to-exec")
                .long("relates-to-exec")
                .help("A command to execute to get the value for the relates to trailer")
                .env("GIT_MIT_RELATES_TO_EXEC")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("relates-to-template")
                .long("relates-to-template")
                .help("A template to apply to the relates to trailer")
                .env("GIT_MIT_RELATES_TO_TEMPLATE")
                .takes_value(true)
                .required(false),
        )
        .arg(Arg::new("completion").long("completion").possible_values(&[
            "bash",
            "elvish",
            "fish",
            "powershell",
            "zsh",
        ]))
}
