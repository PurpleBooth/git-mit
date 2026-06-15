use miette::Result;

use crate::{external::Vcs, mit::Author};
/// # Errors
///
/// On write failure
pub fn set_config_authors(store: &mut dyn Vcs, initial: &str, author: &Author<'_>) -> Result<()> {
    store.set_str(
        &format!("mit.author.config.{initial}.email"),
        author.email(),
    )?;
    store.set_str(&format!("mit.author.config.{initial}.name"), author.name())?;

    if let Some(signingkey) = author.signingkey() {
        store.set_str(
            &format!("mit.author.config.{initial}.signingkey"),
            signingkey,
        )?;
    } else {
        let key = format!("mit.author.config.{initial}.signingkey");
        if store.get_str(&key)?.is_some() {
            store.remove(&key)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use miette::{miette, Result};

    use crate::{
        external::{InMemory, RepoState, Vcs},
        mit::{cmd::set_config_authors::set_config_authors, Author},
    };

    /// A Vcs mock that mimics git2's `Config::remove`, which errors when
    /// the key does not exist (unlike `InMemory` which silently ignores it).
    struct Git2LikeVcs<'a> {
        store: &'a mut BTreeMap<String, String>,
    }

    impl Git2LikeVcs<'_> {
        const fn new(store: &mut BTreeMap<String, String>) -> Git2LikeVcs<'_> {
            Git2LikeVcs { store }
        }
    }

    impl Vcs for Git2LikeVcs<'_> {
        fn entries(&self, _glob: Option<&str>) -> Result<Vec<String>> {
            Ok(vec![])
        }

        fn get_bool(&self, _name: &str) -> Result<Option<bool>> {
            Ok(None)
        }

        fn get_str(&self, name: &str) -> Result<Option<&str>> {
            Ok(self.store.get(name).map(String::as_str))
        }

        fn get_i64(&self, _name: &str) -> Result<Option<i64>> {
            Ok(None)
        }

        fn set_str(&mut self, name: &str, value: &str) -> Result<()> {
            self.store.insert(name.into(), value.into());
            Ok(())
        }

        fn set_i64(&mut self, _name: &str, _value: i64) -> Result<()> {
            Ok(())
        }

        fn remove(&mut self, name: &str) -> Result<()> {
            if self.store.remove(name).is_none() {
                return Err(miette!("could not find key '{name}' to delete"));
            }
            Ok(())
        }

        fn state(&self) -> Option<RepoState> {
            None
        }
    }

    #[test]
    fn can_set_an_author() {
        let mut store: BTreeMap<String, String> = BTreeMap::new();
        let mut vcs = InMemory::new(&mut store);

        set_config_authors(
            &mut vcs,
            "zy",
            &Author::new("Z Y".into(), "zy@example.com".into(), None),
        )
        .expect("command to have succeeded");

        let mut expected: BTreeMap<String, String> = BTreeMap::new();
        expected.insert("mit.author.config.zy.email".into(), "zy@example.com".into());
        expected.insert("mit.author.config.zy.name".into(), "Z Y".into());

        assert_eq!(
            store, expected,
            "Expected the VCS store to contain the author's email and name after setting an author"
        );
    }

    #[test]
    fn can_set_an_author_with_signing_key() {
        let mut store: BTreeMap<String, String> = BTreeMap::new();
        let mut vcs = InMemory::new(&mut store);

        set_config_authors(
            &mut vcs,
            "bt",
            &Author::new(
                "Billie Thompson".into(),
                "billie@example.com".into(),
                Some("ABC".into()),
            ),
        )
        .expect("Should succeed");

        let mut expected: BTreeMap<String, String> = BTreeMap::new();
        expected.insert("mit.author.config.bt.name".into(), "Billie Thompson".into());
        expected.insert(
            "mit.author.config.bt.email".into(),
            "billie@example.com".into(),
        );
        expected.insert("mit.author.config.bt.signingkey".into(), "ABC".into());

        assert_eq!(
            store, expected,
            "Expected the VCS store to contain the author's email, name, and signing key"
        );
    }

    #[test]
    fn updating_author_without_signing_key_removes_old_signing_key() {
        // First, set an author WITH a signing key
        let mut store: BTreeMap<String, String> = BTreeMap::new();
        {
            let mut vcs = InMemory::new(&mut store);

            set_config_authors(
                &mut vcs,
                "bt",
                &Author::new(
                    "Billie Thompson".into(),
                    "billie@example.com".into(),
                    Some("ABC".into()),
                ),
            )
            .expect("Should succeed");
        }

        assert!(
            store.contains_key("mit.author.config.bt.signingkey"),
            "Signing key should be present after first set"
        );

        // Now update the same author WITHOUT a signing key
        {
            let mut vcs = InMemory::new(&mut store);

            set_config_authors(
                &mut vcs,
                "bt",
                &Author::new(
                    "Billie Thompson".into(),
                    "billie@newdomain.com".into(),
                    None,
                ),
            )
            .expect("Should succeed");
        }

        let mut expected: BTreeMap<String, String> = BTreeMap::new();
        expected.insert("mit.author.config.bt.name".into(), "Billie Thompson".into());
        expected.insert(
            "mit.author.config.bt.email".into(),
            "billie@newdomain.com".into(),
        );
        // signingkey should NOT be present

        assert_eq!(
            store, expected,
            "Updating an author without a signing key should remove the old signing key entry"
        );
    }

    #[test]
    fn setting_author_without_signing_key_succeeds_when_no_prior_key_exists() {
        // This reproduces the specdown failure: git2::Config::remove errors
        // when the key doesn't exist, unlike InMemory which silently ignores it.
        let mut store: BTreeMap<String, String> = BTreeMap::new();
        let mut vcs = Git2LikeVcs::new(&mut store);

        set_config_authors(
            &mut vcs,
            "jd",
            &Author::new("Jane Doe".into(), "jd@example.com".into(), None),
        )
        .expect("Should succeed even when no prior signingkey exists");
    }
}
