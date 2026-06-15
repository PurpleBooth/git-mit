use std::collections::BTreeMap;
use std::time::Duration;

use miette::Result;

use crate::external::InMemory;
use crate::mit::{set_commit_authors, Author};

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
        Some("Billie Thompson")
    );
    assert_eq!(
        buffer
            .get("mit.author.coauthors.0.name")
            .map(String::as_str),
        Some("Somebody Else")
    );
    assert_eq!(
        buffer
            .get("mit.author.coauthors.1.name")
            .map(String::as_str),
        Some("Annie Example")
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
        Some("Somebody Else")
    );
    assert_eq!(
        buffer
            .get("mit.author.coauthors.0.name")
            .map(String::as_str),
        Some("Annie Example")
    );
    assert_eq!(
        buffer
            .get("mit.author.coauthors.1.name")
            .map(String::as_str),
        Some("Billie Thompson")
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
        Some("Billie Thompson")
    );
    assert_eq!(
        buffer.get("user.email").map(String::as_str),
        Some("billie@example.com")
    );
    assert!(!buffer.contains_key("mit.author.coauthors.0.name"));

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
    assert!(!buffer.contains_key("user.name"));

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
        Some("Billie Thompson")
    );
    assert_eq!(
        buffer
            .get("mit.author.coauthors.0.name")
            .map(String::as_str),
        Some("Somebody Else")
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
        Some("Somebody Else")
    );
    assert_eq!(
        buffer
            .get("mit.author.coauthors.0.name")
            .map(String::as_str),
        Some("Billie Thompson")
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
        Some("Billie Thompson")
    );
    assert_eq!(
        buffer
            .get("mit.author.coauthors.0.name")
            .map(String::as_str),
        Some("Somebody Else")
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
    assert_eq!(result, original);

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
        Some("Billie Thompson")
    );
    assert_eq!(
        buffer.get("user.email").map(String::as_str),
        Some("billie@example.com")
    );

    Ok(())
}
