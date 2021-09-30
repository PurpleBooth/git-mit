use std::{convert::TryFrom, env};

use miette::{GraphicalTheme, IntoDiagnostic, Result};
use mit_commit_message_lints::{external::Git2, mit::get_commit_coauthor_configuration};

use crate::{cli::app, errors::StaleAuthorError};

fn main() -> Result<()> {
    if env::var("DEBUG_PRETTY_ERRORS").is_ok() {
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .force_graphical(true)
                    .terminal_links(false)
                    .graphical_theme(GraphicalTheme::unicode_nocolor())
                    .build(),
            )
        }))
        .unwrap();
    }
    app().get_matches();

    let current_dir = env::current_dir().into_diagnostic()?;
    let mut git_config = Git2::try_from(current_dir)?;
    let co_author_configuration = get_commit_coauthor_configuration(&mut git_config)?;

    if !co_author_configuration.is_some() {
        return Err(StaleAuthorError {}.into());
    }

    Ok(())
}

mod cli;
mod errors;
