use std::{
    convert::TryInto,
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use miette::{IntoDiagnostic, Result, WrapErr};

use crate::{
    external::Vcs,
    mit::{
        cmd::{errors::Error::NoAuthorsToSet, vcs::has_vcs_coauthor, CONFIG_KEY_EXPIRES},
        Author,
    },
};

/// # Errors
///
/// This errors if writing to the git mit file fails for some reason. Those
/// reasons will be specific to VCS implementation
pub fn set_commit_authors(
    config: &mut dyn Vcs,
    authors: &[&Author<'_>],
    expires_in: Duration,
) -> Result<()> {
    let (first_author, others) = authors.split_first().ok_or(NoAuthorsToSet)?;

    remove_coauthors(config)?;
    set_vcs_user(config, first_author)?;
    set_vcs_coauthors(config, others)?;
    set_vcs_expires_time(config, expires_in)?;

    Ok(())
}

fn remove_coauthors(config: &mut dyn Vcs) -> Result<()> {
    get_defined_vcs_coauthor_keys(config)
        .into_iter()
        .try_for_each(|key| config.remove(&key))?;

    Ok(())
}

#[allow(clippy::maybe_infinite_iter)]
fn get_defined_vcs_coauthor_keys(config: &mut dyn Vcs) -> Vec<String> {
    (0..)
        .take_while(|index| has_vcs_coauthor(config, *index))
        .flat_map(|index| {
            vec![
                format!("mit.author.coauthors.{}.name", index),
                format!("mit.author.coauthors.{}.email", index),
            ]
            .into_iter()
        })
        .map(String::from)
        .collect()
}

fn set_vcs_coauthors(config: &mut dyn Vcs, authors: &[&Author<'_>]) -> Result<()> {
    authors
        .iter()
        .enumerate()
        .try_for_each(|(index, author)| set_vcs_coauthor(config, index, author))
}

fn set_vcs_coauthor(config: &mut dyn Vcs, index: usize, author: &Author<'_>) -> Result<()> {
    set_vcs_coauthor_name(config, index, author)?;
    set_vcs_coauthor_email(config, index, author)?;

    Ok(())
}

fn set_vcs_coauthor_name(config: &mut dyn Vcs, index: usize, author: &Author<'_>) -> Result<()> {
    config.set_str(
        &format!("mit.author.coauthors.{}.name", index),
        author.name(),
    )?;
    Ok(())
}

fn set_vcs_coauthor_email(config: &mut dyn Vcs, index: usize, author: &Author<'_>) -> Result<()> {
    config.set_str(
        &format!("mit.author.coauthors.{}.email", index),
        author.email(),
    )?;
    Ok(())
}

fn set_vcs_user(config: &mut dyn Vcs, author: &Author<'_>) -> Result<()> {
    config.set_str("user.name", author.name())?;
    config.set_str("user.email", author.email())?;
    set_author_signing_key(config, author)?;

    Ok(())
}

fn set_author_signing_key(config: &mut dyn Vcs, author: &Author<'_>) -> Result<()> {
    match author.signingkey() {
        Some(key) => config
            .set_str("user.signingkey", key)
            .wrap_err("failed to set git author's signing key "),
        None => config.remove("user.signingkey").or(Ok(())),
    }
}

fn set_vcs_expires_time(config: &mut dyn Vcs, expires_in: Duration) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .into_diagnostic()?;
    let expiry_time = now.add(expires_in).as_secs().try_into().into_diagnostic()?;
    config
        .set_i64(CONFIG_KEY_EXPIRES, expiry_time)
        .wrap_err("failed to set author expiry name")
}
