use miette::Result;
use rand::seq::SliceRandom;

use crate::external::Vcs;
use crate::mit::cmd::set_commit_authors::{remove_coauthors, set_vcs_coauthor, set_vcs_user};
use crate::mit::{cmd::vcs::get_vcs_coauthors_config, Author};

/// Rotate the primary author among configured authors
///
/// Moves the first coauthor to become the primary author,
/// and demotes the current primary to be the last coauthor.
/// This affects the NEXT commit (git reads user.name/email
/// before the prepare-commit-msg hook runs).
///
/// # Errors
///
/// Returns an error if:
/// - Reading git config (user.name, user.email, or coauthors) fails
/// - Writing git config (removing old coauthors or setting new authors) fails
///
/// # Panics
///
/// This function will panic if the primary author (constructed from user.name
/// and user.email) is None. However, this is prevented by an early return check:
/// if either user.name or user.email is missing, the function returns Ok(())
/// without attempting to unwrap. The unwrap on line 47 is safe because at that
/// point we've confirmed both name and email exist.
pub fn rotate_authors(config: &mut dyn Vcs, strategy: crate::mit::RotationOption) -> Result<()> {
    // Read the current primary author
    let primary_name = config.get_str("user.name")?.map(String::from);
    let primary_email = config.get_str("user.email")?.map(String::from);
    let primary_signingkey = config.get_str("user.signingkey")?.map(String::from);

    let primary = match (primary_name, primary_email, primary_signingkey) {
        (Some(name), Some(email), signingkey) => Some(Author::new(
            name.into(),
            email.into(),
            signingkey.map(Into::into),
        )),
        _ => return Ok(()), // No primary author, nothing to rotate
    };

    // Read coauthors
    let coauthor_emails: Vec<String> = get_vcs_coauthors_config(config, "email")?
        .into_iter()
        .filter_map(|x| x.map(|s| s.to_string()))
        .collect();

    let coauthors: Vec<Author> = get_vcs_coauthors_config(config, "name")?
        .into_iter()
        .filter_map(|x| x.map(|s| s.to_string()))
        .zip(coauthor_emails)
        .filter_map(|(name, email)| {
            if name.is_empty() || email.is_empty() {
                None
            } else {
                Some(Author::new(name.into(), email.into(), None))
            }
        })
        .collect();

    // Build full author list
    let mut all_authors: Vec<Author> = vec![primary.unwrap()];
    all_authors.extend(coauthors);

    // If only 0 or 1 author, nothing to rotate
    if all_authors.len() <= 1 {
        return Ok(());
    }

    // Apply the rotation strategy
    match strategy {
        crate::mit::RotationOption::RoundRobin => {
            all_authors.rotate_left(1);
        }
        crate::mit::RotationOption::Random => {
            all_authors.shuffle(&mut rand::rng());
        }
    }

    // Write back
    remove_coauthors(config)?;
    set_vcs_user(config, &all_authors[0])?;
    all_authors[1..]
        .iter()
        .enumerate()
        .try_for_each(|(index, author)| set_vcs_coauthor(config, index, author))?;

    Ok(())
}
