use std::collections::BTreeMap;

use crate::external::Vcs;
use crate::mit::{Author, Authors, VcsError};

/// # Errors
///
/// On IO failure
///
/// # Panics
///
/// Does not panic
pub fn get_config_authors(vcs: &dyn Vcs) -> Result<Authors, VcsError> {
    let raw_entries: BTreeMap<String, BTreeMap<String, String>> = vcs
        .entries(Some("mit.author.config.*"))?
        .iter()
        .map(|x| (x, x.trim_start_matches("mit.author.config.")))
        .map(|(x, y)| (x, y.split_terminator('.').collect::<Vec<_>>()))
        .try_fold::<_, _, Result<_, VcsError>>(BTreeMap::new(), |mut acc, (key, fragments)| {
            let mut fragment_iterator = fragments.iter();
            let initial = String::from(*fragment_iterator.next().unwrap());
            let part = String::from(*fragment_iterator.next().unwrap());

            let mut existing: BTreeMap<String, String> =
                acc.get(&initial).map(BTreeMap::clone).unwrap_or_default();
            existing.insert(part, String::from(vcs.get_str(key)?.unwrap()));

            acc.insert(initial, existing);
            Ok(acc)
        })?;

    Ok(Authors::new(
        raw_entries
            .iter()
            .filter_map(|(key, y)| {
                let name = y.get("name").map(String::clone);
                let email = y.get("email").map(String::clone);
                let signingkey: Option<String> = y.get("signingkey").map(String::clone);

                match (name, email, signingkey) {
                    (Some(name), Some(email), None) => {
                        Some((key, Author::new(&name, &email, None)))
                    }
                    (Some(name), Some(email), Some(signingkey)) => {
                        Some((key, Author::new(&name, &email, Some(&signingkey))))
                    }
                    _ => None,
                }
            })
            .fold(
                BTreeMap::new(),
                |mut acc: BTreeMap<String, Author>, (key, value): (&String, Author)| {
                    acc.insert(key.clone(), value);
                    acc
                },
            ),
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::mit::cmd::get_config_authors::get_config_authors;
    use crate::mit::Authors;
    use crate::{external::InMemory, mit::Author};

    #[test]
    fn it_can_give_me_an_author() {
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert("mit.author.config.zy.email".into(), "zy@example.com".into());
        strings.insert("mit.author.config.zy.name".into(), "Z Y".into());
        let vcs = InMemory::new(&mut strings);

        let actual = get_config_authors(&vcs).expect("Failed to read VCS config");
        let expected_author = Author::new("Z Y", "zy@example.com", None);
        let mut store = BTreeMap::new();
        store.insert("zy".into(), expected_author);
        let expected = Authors::new(store);
        assert_eq!(
            expected, actual,
            "Expected the mit config to be {:?}, instead got {:?}",
            expected, actual
        );
    }

    #[test]
    fn it_can_give_me_multiple_authors() {
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert("mit.author.config.zy.email".into(), "zy@example.com".into());
        strings.insert("mit.author.config.zy.name".into(), "Z Y".into());
        strings.insert(
            "mit.author.config.bt.email".into(),
            "billie@example.com".into(),
        );
        strings.insert("mit.author.config.bt.name".into(), "Billie Thompson".into());
        strings.insert("mit.author.config.bt.signingkey".into(), "ABC".into());
        let vcs = InMemory::new(&mut strings);

        let actual = get_config_authors(&vcs).expect("Failed to read VCS config");
        let mut store = BTreeMap::new();
        store.insert("zy".into(), Author::new("Z Y", "zy@example.com", None));
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", Some("ABC")),
        );
        let expected = Authors::new(store);
        assert_eq!(
            expected, actual,
            "Expected the mit config to be {:?}, instead got {:?}",
            expected, actual
        );
    }

    #[test]
    fn broken_authors_are_skipped() {
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert("mit.author.config.zy.name".into(), "Z Y".into());
        strings.insert(
            "mit.author.config.bt.email".into(),
            "billie@example.com".into(),
        );
        strings.insert("mit.author.config.bt.name".into(), "Billie Thompson".into());
        strings.insert("mit.author.config.bt.signingkey".into(), "ABC".into());
        let vcs = InMemory::new(&mut strings);

        let actual = get_config_authors(&vcs).expect("Failed to read VCS config");
        let mut store = BTreeMap::new();
        store.insert(
            "bt".into(),
            Author::new("Billie Thompson", "billie@example.com", Some("ABC")),
        );
        let expected = Authors::new(store);
        assert_eq!(
            expected, actual,
            "Expected the mit config to be {:?}, instead got {:?}",
            expected, actual
        );
    }
}
