use hook::{dir, install};
use indoc::indoc;

pub(crate) use crate::cli::app;
use crate::cli::args::Args;

mod cli;
mod errors;
mod hook;
use std::env;

use miette::Result;

fn main() -> Result<()> {
    if env::var("DEBUG_PRETTY_ERRORS").is_ok() {
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .force_graphical(true)
                    .build(),
            )
        }))
        .unwrap();
    }

    let args: Args = app::app().get_matches().into();

    let hooks = dir::create(args.scope().is_global())?;

    install::link(&hooks, "prepare-commit-msg")?;
    install::link(&hooks, "pre-commit")?;
    install::link(&hooks, "commit-msg")?;

    if args.scope().is_global() {
        mit_commit_message_lints::console::style::success(
            "git-mit will be added for newly created or cloned repositories",
            "inside existing repositories run \"git init\" to set them up",
        );
    } else {
        mit_commit_message_lints::console::style::success(
            "git-mit is setup for the current repository",
            indoc! {r#"
                Optionally you can install git-mit for all new "git clone" and "git init" commands

                git mit-install --scope=global
            "#},
        );
    }

    mit_commit_message_lints::console::style::success(
        "Adding your first pairing partners",
        indoc! {r#"
            git mit-config mit set bt "Billie Thompson" billie@example.com
            git mit-config mit set se "Someone Else" someone@example.com

            To add multiple users to your commit run. Remember to include yourself!

            git mit bt se

            Optionally you can also add a issue number by running

            git mit-relates-to "[#134]"

            When you can use the "-m" flag or your editor, both work as normal

            git commit -m "Your message"

            The authors and issue number appear on the commit. These authors are saved into your current repository for ad-hoc pairing. When you're ready make the authors everywhere run

            mkdir -p "$HOME/.config/git-mit"
            git mit-config generate > "$HOME/.config/git-mit/mit.toml"
        "#},
    );

    Ok(())
}
