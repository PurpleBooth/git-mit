use miette::Result;
use rand::seq::SliceRandom;

use crate::external::Vcs;
use crate::mit::cmd::set_commit_authors::{remove_coauthors, set_vcs_coauthor, set_vcs_user};
use crate::mit::{Author, cmd::vcs::get_vcs_coauthors_config};

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
        crate::mit::RotationOption::Off => return Ok(()),
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::time::Duration;

    use miette::Result;

    use crate::external::InMemory;
    use crate::mit::{Author, set_commit_authors};

    #[test]
    fn rotate_authors_rotates_three_authors() -> Result<()> {
        let mut buffer = BTreeMap::new();
        {
            let mut vcs_config = InMemory::new(&mut buffer);

            let author_1 = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
            let author_2 = Author::new("Somebody Else".into(), "someone@example.com".into(), None);
            let author_3 = Author::new("Annie Example".into(), "annie@example.com".into(), None);

            set_commit_authors(
                &mut vcs_config,
                &[&author_1, &author_2, &author_3],
                Duration::from_hours(1),
            )?;
        }

        // Initial state: A is primary, B & C are coauthors
        assert_eq!(
            buffer.get("user.name").map(String::as_str),
            Some("Billie Thompson"),
            "Expected the initial primary author to be Billie Thompson"
        );
        assert_eq!(
            buffer
                .get("mit.author.coauthors.0.name")
                .map(String::as_str),
            Some("Somebody Else"),
            "Expected the first coauthor to be Somebody Else before rotation"
        );
        assert_eq!(
            buffer
                .get("mit.author.coauthors.1.name")
                .map(String::as_str),
            Some("Annie Example"),
            "Expected the second coauthor to be Annie Example before rotation"
        );

        // Rotate: B becomes primary, C & A are coauthors
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            crate::mit::cmd::rotate_authors::rotate_authors(
                &mut vcs_config,
                crate::mit::RotationOption::RoundRobin,
            )?;
        }

        assert_eq!(
            buffer.get("user.name").map(String::as_str),
            Some("Somebody Else"),
            "Expected the primary author to be Somebody Else after rotation"
        );
        assert_eq!(
            buffer
                .get("mit.author.coauthors.0.name")
                .map(String::as_str),
            Some("Annie Example"),
            "Expected the first coauthor to be Annie Example after rotation"
        );
        assert_eq!(
            buffer
                .get("mit.author.coauthors.1.name")
                .map(String::as_str),
            Some("Billie Thompson"),
            "Expected the second coauthor to be Billie Thompson after rotation"
        );

        Ok(())
    }

    #[test]
    fn rotate_authors_noops_with_single_author() -> Result<()> {
        let mut buffer = BTreeMap::new();
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
            set_commit_authors(&mut vcs_config, &[&author], Duration::from_hours(1))?;
        }

        {
            let mut vcs_config = InMemory::new(&mut buffer);
            crate::mit::cmd::rotate_authors::rotate_authors(
                &mut vcs_config,
                crate::mit::RotationOption::RoundRobin,
            )?;
        }

        // Should be unchanged
        assert_eq!(
            buffer.get("user.name").map(String::as_str),
            Some("Billie Thompson"),
            "Expected user.name to be unchanged with a single author"
        );
        assert_eq!(
            buffer.get("user.email").map(String::as_str),
            Some("billie@example.com"),
            "Expected user.email to be unchanged with a single author"
        );
        assert!(
            !buffer.contains_key("mit.author.coauthors.0.name"),
            "Expected no coauthors to be set with a single author"
        );

        Ok(())
    }

    #[test]
    fn rotate_authors_noops_with_zero_authors() -> Result<()> {
        let mut buffer = BTreeMap::new();

        {
            let mut vcs_config = InMemory::new(&mut buffer);
            crate::mit::cmd::rotate_authors::rotate_authors(
                &mut vcs_config,
                crate::mit::RotationOption::RoundRobin,
            )?;
        }

        // Buffer should be unchanged
        assert!(
            !buffer.contains_key("user.name"),
            "Expected no user.name to be set when there are no authors"
        );

        Ok(())
    }

    #[test]
    fn rotate_authors_rotates_two_authors() -> Result<()> {
        let mut buffer = BTreeMap::new();
        {
            let mut vcs_config = InMemory::new(&mut buffer);

            let author_1 = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
            let author_2 = Author::new("Somebody Else".into(), "someone@example.com".into(), None);

            set_commit_authors(
                &mut vcs_config,
                &[&author_1, &author_2],
                Duration::from_hours(1),
            )?;
        }

        // Initial: A is primary, B is coauthor
        assert_eq!(
            buffer.get("user.name").map(String::as_str),
            Some("Billie Thompson"),
            "Expected the initial primary author to be Billie Thompson"
        );
        assert_eq!(
            buffer
                .get("mit.author.coauthors.0.name")
                .map(String::as_str),
            Some("Somebody Else"),
            "Expected the first coauthor to be Somebody Else before rotation"
        );

        // Rotate: B becomes primary, A becomes coauthor
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            crate::mit::cmd::rotate_authors::rotate_authors(
                &mut vcs_config,
                crate::mit::RotationOption::RoundRobin,
            )?;
        }

        assert_eq!(
            buffer.get("user.name").map(String::as_str),
            Some("Somebody Else"),
            "Expected the primary author to be Somebody Else after first rotation"
        );
        assert_eq!(
            buffer
                .get("mit.author.coauthors.0.name")
                .map(String::as_str),
            Some("Billie Thompson"),
            "Expected the first coauthor to be Billie Thompson after first rotation"
        );

        // Rotate again: back to A primary, B coauthor
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            crate::mit::cmd::rotate_authors::rotate_authors(
                &mut vcs_config,
                crate::mit::RotationOption::RoundRobin,
            )?;
        }

        assert_eq!(
            buffer.get("user.name").map(String::as_str),
            Some("Billie Thompson"),
            "Expected the primary author to be Billie Thompson after second rotation"
        );
        assert_eq!(
            buffer
                .get("mit.author.coauthors.0.name")
                .map(String::as_str),
            Some("Somebody Else"),
            "Expected the first coauthor to be Somebody Else after second rotation"
        );

        Ok(())
    }

    #[test]
    fn rotate_authors_random_produces_valid_permutation() -> Result<()> {
        let mut buffer = BTreeMap::new();
        {
            let mut vcs_config = InMemory::new(&mut buffer);

            let author_1 = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
            let author_2 = Author::new("Somebody Else".into(), "someone@example.com".into(), None);
            let author_3 = Author::new("Annie Example".into(), "annie@example.com".into(), None);

            set_commit_authors(
                &mut vcs_config,
                &[&author_1, &author_2, &author_3],
                Duration::from_hours(1),
            )?;
        }

        // Collect original author set (sorted)
        let mut original: Vec<String> = vec![buffer.get("user.name").cloned().unwrap_or_default()];
        for i in 0..3 {
            if let Some(name) = buffer.get(&format!("mit.author.coauthors.{i}.name")) {
                original.push(name.clone());
            }
        }
        original.sort();

        // Rotate randomly
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            crate::mit::cmd::rotate_authors::rotate_authors(
                &mut vcs_config,
                crate::mit::RotationOption::Random,
            )?;
        }

        // Collect result author set (sorted)
        let mut result: Vec<String> = vec![buffer.get("user.name").cloned().unwrap_or_default()];
        for i in 0..3 {
            if let Some(name) = buffer.get(&format!("mit.author.coauthors.{i}.name")) {
                result.push(name.clone());
            }
        }
        result.sort();

        // The multiset of authors must be preserved (it's a permutation)
        assert_eq!(
            result, original,
            "Expected random rotation to produce a valid permutation of the original authors"
        );

        Ok(())
    }

    #[test]
    fn rotate_authors_random_noops_with_single_author() -> Result<()> {
        let mut buffer = BTreeMap::new();
        {
            let mut vcs_config = InMemory::new(&mut buffer);
            let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
            set_commit_authors(&mut vcs_config, &[&author], Duration::from_hours(1))?;
        }

        {
            let mut vcs_config = InMemory::new(&mut buffer);
            crate::mit::cmd::rotate_authors::rotate_authors(
                &mut vcs_config,
                crate::mit::RotationOption::Random,
            )?;
        }

        assert_eq!(
            buffer.get("user.name").map(String::as_str),
            Some("Billie Thompson"),
            "Expected user.name to be unchanged with a single author under random rotation"
        );
        assert_eq!(
            buffer.get("user.email").map(String::as_str),
            Some("billie@example.com"),
            "Expected user.email to be unchanged with a single author under random rotation"
        );

        Ok(())
    }

    #[test]
    fn rotate_authors_ignores_coauthor_with_empty_name() -> Result<()> {
        let mut buffer = BTreeMap::new();
        buffer.insert("user.name".into(), "Billie Thompson".into());
        buffer.insert("user.email".into(), "billie@example.com".into());
        // Coauthor with empty name but non-empty email — should be filtered out
        buffer.insert("mit.author.coauthors.0.name".into(), String::new());
        buffer.insert(
            "mit.author.coauthors.0.email".into(),
            "ghost@example.com".into(),
        );

        {
            let mut vcs_config = InMemory::new(&mut buffer);
            crate::mit::cmd::rotate_authors::rotate_authors(
                &mut vcs_config,
                crate::mit::RotationOption::RoundRobin,
            )?;
        }

        // With the empty-name coauthor filtered, only the primary remains,
        // so rotation should be a no-op.
        assert_eq!(
            buffer.get("user.name").map(String::as_str),
            Some("Billie Thompson"),
            "Expected user.name to be unchanged when the only coauthor has an empty name"
        );

        Ok(())
    }
}
