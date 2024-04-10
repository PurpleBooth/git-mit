use std::borrow::Cow;

use miette::{IntoDiagnostic, Result};
use time::OffsetDateTime;

use crate::{
    external::Vcs,
    mit::{cmd::CONFIG_KEY_EXPIRES, Author, AuthorState},
};

/// Get the co-authors that are currently defined for this vcs config source
///
/// # Errors
///
/// Will fail if reading or writing from the VCS config fails, or it contains
/// data in an incorrect format
pub fn get_commit_coauthor_configuration(config: &dyn Vcs) -> Result<AuthorState<Vec<Author<'_>>>> {
    let config_value = config.get_i64(CONFIG_KEY_EXPIRES)?;

    match config_value {
        Some(config_value) => {
            let config_time =
                OffsetDateTime::from_unix_timestamp(config_value).into_diagnostic()?;
            if OffsetDateTime::now_utc() < config_time {
                let author_config = get_vcs_authors(config)?;

                Ok(AuthorState::Some(author_config))
            } else {
                Ok(AuthorState::Timeout(config_time))
            }
        }
        None => Ok(AuthorState::None),
    }
}

fn get_vcs_authors(config: &'_ dyn Vcs) -> Result<Vec<Author<'_>>> {
    let co_author_names = get_vcs_coauthor_names(config)?;
    let co_author_emails = get_vcs_coauthor_emails(config)?;

    Ok(co_author_names
        .into_iter()
        .zip(co_author_emails)
        .filter_map(new_author)
        .collect())
}

fn new_author<'a>(parameters: (Option<Cow<'a, str>>, Option<Cow<'a, str>>)) -> Option<Author<'a>> {
    match parameters {
        (Some(name), Some(email)) => Some(Author::new(name, email, None)),
        _ => None,
    }
}

fn get_vcs_coauthor_names(config: &'_ dyn Vcs) -> Result<Vec<Option<Cow<'_, str>>>> {
    super::vcs::get_vcs_coauthors_config(config, "name")
}

fn get_vcs_coauthor_emails(config: &'_ dyn Vcs) -> Result<Vec<Option<Cow<'_, str>>>> {
    super::vcs::get_vcs_coauthors_config(config, "email")
}
