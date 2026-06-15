use std::collections::BTreeMap;

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

        set_commit_authors(&mut vcs_config, &[&author_1, &author_2, &author_3], std::time::Duration::from_secs(3600))?;
    }

    // Initial state: A is primary, B & C are coauthors
    assert_eq!(buffer.get("user.name").map(|s| s.as_str()), Some("Billie Thompson"));
    assert_eq!(buffer.get("mit.author.coauthors.0.name").map(|s| s.as_str()), Some("Somebody Else"));
    assert_eq!(buffer.get("mit.author.coauthors.1.name").map(|s| s.as_str()), Some("Annie Example"));

    // Rotate: B becomes primary, C & A are coauthors
    {
        let mut vcs_config = InMemory::new(&mut buffer);
        crate::mit::cmd::rotate_authors::rotate_authors(&mut vcs_config)?;
    }

    assert_eq!(buffer.get("user.name").map(|s| s.as_str()), Some("Somebody Else"));
    assert_eq!(buffer.get("mit.author.coauthors.0.name").map(|s| s.as_str()), Some("Annie Example"));
    assert_eq!(buffer.get("mit.author.coauthors.1.name").map(|s| s.as_str()), Some("Billie Thompson"));

    Ok(())
}

#[test]
fn rotate_authors_noops_with_single_author() -> Result<()> {
    let mut buffer = BTreeMap::new();
    {
        let mut vcs_config = InMemory::new(&mut buffer);
        let author = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
        set_commit_authors(&mut vcs_config, &[&author], std::time::Duration::from_secs(3600))?;
    }

    {
        let mut vcs_config = InMemory::new(&mut buffer);
        crate::mit::cmd::rotate_authors::rotate_authors(&mut vcs_config)?;
    }

    // Should be unchanged
    assert_eq!(buffer.get("user.name").map(|s| s.as_str()), Some("Billie Thompson"));
    assert_eq!(buffer.get("user.email").map(|s| s.as_str()), Some("billie@example.com"));
    assert!(buffer.get("mit.author.coauthors.0.name").is_none());

    Ok(())
}

#[test]
fn rotate_authors_noops_with_zero_authors() -> Result<()> {
    let mut buffer = BTreeMap::new();

    {
        let mut vcs_config = InMemory::new(&mut buffer);
        crate::mit::cmd::rotate_authors::rotate_authors(&mut vcs_config)?;
    }

    // Buffer should be unchanged
    assert!(buffer.get("user.name").is_none());

    Ok(())
}

#[test]
fn rotate_authors_rotates_two_authors() -> Result<()> {
    let mut buffer = BTreeMap::new();
    {
        let mut vcs_config = InMemory::new(&mut buffer);

        let author_1 = Author::new("Billie Thompson".into(), "billie@example.com".into(), None);
        let author_2 = Author::new("Somebody Else".into(), "someone@example.com".into(), None);

        set_commit_authors(&mut vcs_config, &[&author_1, &author_2], std::time::Duration::from_secs(3600))?;
    }

    // Initial: A is primary, B is coauthor
    assert_eq!(buffer.get("user.name").map(|s| s.as_str()), Some("Billie Thompson"));
    assert_eq!(buffer.get("mit.author.coauthors.0.name").map(|s| s.as_str()), Some("Somebody Else"));

    // Rotate: B becomes primary, A becomes coauthor
    {
        let mut vcs_config = InMemory::new(&mut buffer);
        crate::mit::cmd::rotate_authors::rotate_authors(&mut vcs_config)?;
    }

    assert_eq!(buffer.get("user.name").map(|s| s.as_str()), Some("Somebody Else"));
    assert_eq!(buffer.get("mit.author.coauthors.0.name").map(|s| s.as_str()), Some("Billie Thompson"));

    // Rotate again: back to A primary, B coauthor
    {
        let mut vcs_config = InMemory::new(&mut buffer);
        crate::mit::cmd::rotate_authors::rotate_authors(&mut vcs_config)?;
    }

    assert_eq!(buffer.get("user.name").map(|s| s.as_str()), Some("Billie Thompson"));
    assert_eq!(buffer.get("mit.author.coauthors.0.name").map(|s| s.as_str()), Some("Somebody Else"));

    Ok(())
}