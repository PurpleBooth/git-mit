use crate::external::Vcs;
use crate::mit::{Author, VcsError};

/// # Errors
///
/// On write failure
pub fn set_config_authors(
    store: &mut dyn Vcs,
    initial: &str,
    author: &Author,
) -> Result<(), VcsError> {
    store.set_str(
        &format!("mit.author.config.{}.email", initial),
        &author.email(),
    )?;
    store.set_str(
        &format!("mit.author.config.{}.name", initial),
        &author.name(),
    )?;

    if let Some(signingkey) = author.signingkey() {
        store.set_str(
            &format!("mit.author.config.{}.signingkey", initial),
            &signingkey,
        )?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::external::InMemory;
    use crate::mit::cmd::set_config_authors::set_config_authors;
    use crate::mit::Author;
    use std::collections::BTreeMap;

    #[test]
    fn can_set_an_author() {
        let mut store: BTreeMap<String, String> = BTreeMap::new();
        let mut vcs = InMemory::new(&mut store);

        set_config_authors(&mut vcs, "zy", &Author::new("Z Y", "zy@example.com", None))
            .expect("command to have succeeded");

        let mut expected: BTreeMap<String, String> = BTreeMap::new();
        expected.insert("mit.author.config.zy.email".into(), "zy@example.com".into());
        expected.insert("mit.author.config.zy.name".into(), "Z Y".into());

        assert_eq!(store, expected);
    }

    #[test]
    fn can_set_an_author_with_signing_key() {
        let mut store: BTreeMap<String, String> = BTreeMap::new();
        let mut vcs = InMemory::new(&mut store);

        set_config_authors(
            &mut vcs,
            "bt",
            &Author::new("Billie Thompson", "billie@example.com", Some("ABC")),
        )
        .expect("Should succeed");

        let mut expected: BTreeMap<String, String> = BTreeMap::new();
        expected.insert("mit.author.config.bt.name".into(), "Billie Thompson".into());
        expected.insert(
            "mit.author.config.bt.email".into(),
            "billie@example.com".into(),
        );
        expected.insert("mit.author.config.bt.signingkey".into(), "ABC".into());

        assert_eq!(store, expected);
    }
}
