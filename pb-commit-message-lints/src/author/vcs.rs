use std::{
    convert::TryFrom,
    error::Error,
    ops::Add,
    option::Option,
    result::Result,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{author::entities::Author, external::vcs::Vcs};

const CONFIG_KEY_EXPIRES: &str = "pb.author.expires";

#[must_use]
pub fn get_coauthor_configuration(config: &dyn Vcs) -> Option<Vec<Author>> {
    config
        .get_i64(CONFIG_KEY_EXPIRES)
        .ok_or_else(|| "No author expiry date".into())
        .and_then(i64_into_u64)
        .map(Duration::from_secs)
        .and_then(join_time_and_now)
        .map(|(point, comparison)| point.lt(&comparison))
        .ok()
        .filter(bool::clone)
        .map(|_| get_vcs_authors(config))
}

fn join_time_and_now(expires_after_time: Duration) -> Result<(Duration, Duration), Box<dyn Error>> {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(Box::from)
        .map(|time_since_epoch| (time_since_epoch, expires_after_time))
}

fn i64_into_u64(input: i64) -> Result<u64, Box<dyn Error>> {
    u64::try_from(input).map_err(Box::<dyn Error>::from)
}

fn u64_into_i64(input: u64) -> Result<i64, Box<dyn Error>> {
    i64::try_from(input).map_err(Box::<dyn Error>::from)
}

fn get_vcs_authors(config: &dyn Vcs) -> Vec<Author> {
    get_vcs_coauthor_names(config)
        .iter()
        .zip(get_vcs_coauthor_emails(config))
        .filter_map(new_author)
        .collect()
}

fn new_author(parameters: (&Option<&str>, Option<&str>)) -> Option<Author> {
    match parameters {
        (Some(name), Some(email)) => Some(Author::new(name, email, None)),
        _ => None,
    }
}

fn get_vcs_coauthor_names(config: &dyn Vcs) -> Vec<Option<&str>> {
    get_vcs_coauthors_config(config, "name")
}

fn get_vcs_coauthor_emails(config: &dyn Vcs) -> Vec<Option<&str>> {
    get_vcs_coauthors_config(config, "email")
}

#[allow(clippy::maybe_infinite_iter)]
fn get_vcs_coauthors_config<'a>(config: &'a dyn Vcs, key: &str) -> Vec<Option<&'a str>> {
    (0..)
        .take_while(|index| has_vcs_coauthor(config, *index))
        .map(|index| get_vcs_coauthor_config(config, key, index))
        .collect()
}

fn get_vcs_coauthor_config<'a>(config: &'a dyn Vcs, key: &str, index: i32) -> Option<&'a str> {
    config.get_str(&format!("pb.author.coauthors.{}.{}", index, key))
}

fn has_vcs_coauthor(config: &dyn Vcs, index: i32) -> bool {
    get_vcs_coauthor_config(config, "email", index)
        .and_then(|_| get_vcs_coauthor_config(config, "name", index))
        .is_some()
}

#[cfg(test)]
mod tests_able_to_load_config_from_git {
    use std::{
        collections::HashMap,
        convert::TryFrom,
        ops::{Add, Sub},
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use pretty_assertions::assert_eq;

    use crate::{
        author::{entities::Author, vcs::get_coauthor_configuration},
        external::vcs::InMemory,
    };

    #[test]
    fn there_is_no_author_config_if_it_has_expired() {
        let now_minus_10 = epoch_with_offset(subtract_10_seconds);
        let mut bools: HashMap<String, bool> = HashMap::new();
        let mut strings: HashMap<String, String> = HashMap::new();
        let mut i64s = HashMap::new();
        i64s.insert("pb.author.expires".into(), now_minus_10);
        let vcs = InMemory::new(&mut bools, &mut strings, &mut i64s);

        let actual = get_coauthor_configuration(&vcs);
        let expected = None;
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn there_is_a_config_if_the_config_has_not_expired() {
        let mut i64s = HashMap::new();
        i64s.insert(
            "pb.author.expires".into(),
            epoch_with_offset(add_10_seconds),
        );

        let mut bools = HashMap::new();
        let mut strings = HashMap::new();
        let vcs = InMemory::new(&mut bools, &mut strings, &mut i64s);

        let actual = get_coauthor_configuration(&vcs);
        let expected: Option<Vec<Author>> = Some(vec![]);

        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn we_get_author_config_back_if_there_is_any() {
        let mut i64s = HashMap::new();
        i64s.insert(
            "pb.author.expires".into(),
            epoch_with_offset(add_10_seconds),
        );
        let mut strs = HashMap::new();
        strs.insert(
            "pb.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        strs.insert("pb.author.coauthors.0.name".into(), "Annie Example".into());
        let mut bools = HashMap::new();
        let vcs = InMemory::new(&mut bools, &mut strs, &mut i64s);

        let actual = get_coauthor_configuration(&vcs);
        let expected = Some(vec![Author::new(
            "Annie Example",
            "annie@example.com",
            None,
        )]);

        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    fn add_10_seconds(x: Duration) -> Duration {
        x.add(Duration::from_secs(10))
    }

    fn subtract_10_seconds(x: Duration) -> Duration {
        x.sub(Duration::from_secs(10))
    }

    fn into_seconds(x: Duration) -> u64 {
        x.as_secs()
    }

    #[test]
    fn we_get_multiple_authors_back_if_there_are_multiple() {
        let mut i64s = HashMap::new();
        i64s.insert(
            "pb.author.expires".into(),
            epoch_with_offset(add_10_seconds),
        );

        let mut strs = HashMap::new();
        strs.insert(
            "pb.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        strs.insert("pb.author.coauthors.0.name".into(), "Annie Example".into());
        strs.insert(
            "pb.author.coauthors.1.email".into(),
            "joe@example.com".into(),
        );
        strs.insert("pb.author.coauthors.1.name".into(), "Joe Bloggs".into());

        let mut bools = HashMap::new();
        let vcs = InMemory::new(&mut bools, &mut strs, &mut i64s);

        let actual = get_coauthor_configuration(&vcs);
        let expected = Some(vec![
            Author::new("Annie Example", "annie@example.com", None),
            Author::new("Joe Bloggs", "joe@example.com", None),
        ]);

        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    fn epoch_with_offset(x: fn(Duration) -> Duration) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(x)
            .map(into_seconds)
            .map(i64::try_from)
            .expect("Failed to get Unix Epoch")
            .expect("Convert epoch to int")
    }
}

///
/// # Errors
///
/// This errors if writing to the git authors file fails for some reason. Those reasons will be specific to VCS implementation
pub fn set_authors<'a>(
    config: &'a mut dyn Vcs,
    authors: &[&Author],
    expires_in: Duration,
) -> Result<(), Box<dyn Error>> {
    authors
        .split_first()
        .ok_or_else(|| "Needs at least one author".into())
        .and_then(|(first, others)| set_vcs_user(config, first).map(|_| others))
        .and_then(|authors| set_vcs_coauthors(config, authors))
        .and_then(|_| set_vcs_expires_time(config, expires_in))
}

fn set_vcs_coauthors(config: &mut dyn Vcs, authors: &[&Author]) -> Result<(), Box<dyn Error>> {
    authors
        .iter()
        .enumerate()
        .try_for_each(|(index, author)| set_vcs_coauthor(config, index, author))
}

fn set_vcs_coauthor(
    config: &mut dyn Vcs,
    index: usize,
    author: &Author,
) -> Result<(), Box<dyn Error>> {
    set_vcs_coauthor_name(config, index, author)
        .and_then(|_| set_vcs_coauthor_email(config, index, author))
}

fn set_vcs_coauthor_name(
    config: &mut dyn Vcs,
    index: usize,
    author: &Author,
) -> Result<(), Box<dyn Error>> {
    config
        .set_str(
            &format!("pb.author.coauthors.{}.name", index),
            &author.name(),
        )
        .map_err(Box::<dyn Error>::from)
}

fn set_vcs_coauthor_email(
    config: &mut dyn Vcs,
    index: usize,
    author: &Author,
) -> Result<(), Box<dyn Error>> {
    config
        .set_str(
            &format!("pb.author.coauthors.{}.email", index),
            &author.email(),
        )
        .map_err(Box::<dyn Error>::from)
}

fn set_vcs_user(config: &mut dyn Vcs, author: &Author) -> Result<(), Box<dyn Error>> {
    config
        .set_str("user.name", &author.name())
        .and_then(|_| config.set_str("user.email", &author.email()))
        .and_then(|_| set_author_signing_key(config, author))
}

fn set_author_signing_key(config: &mut dyn Vcs, author: &Author) -> Result<(), Box<dyn Error>> {
    match author.signingkey() {
        Some(key) => config.set_str("user.signingkey", &key),
        None => config.remove("user.signingkey").or_else(|_| Ok(())),
    }
}

fn set_vcs_expires_time(config: &mut dyn Vcs, expires_in: Duration) -> Result<(), Box<dyn Error>> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(Box::from)
        .map(|now| now.add(expires_in))
        .map(|expiry_time| expiry_time.as_secs())
        .and_then(u64_into_i64)
        .and_then(|expires_time| config.set_i64(CONFIG_KEY_EXPIRES, expires_time))
}

#[cfg(test)]
mod tests_can_set_author_details {
    use std::{
        collections::HashMap,
        convert::TryFrom,
        error::Error,
        ops::Add,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use crate::{
        author::{entities::Author, vcs::set_authors},
        external::vcs::InMemory,
    };

    #[test]
    fn the_first_initial_becomes_the_author() {
        let mut i64s = HashMap::new();
        let mut strs = HashMap::new();
        let mut bools = HashMap::new();

        let mut vcs_config = InMemory::new(&mut bools, &mut strs, &mut i64s);

        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let actual = set_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(Some(&"Billie Thompson".to_string()), strs.get("user.name"));
        assert_eq!(
            Some(&"billie@example.com".to_string()),
            strs.get("user.email")
        )
    }

    #[test]
    fn the_first_initial_sets_signing_key_if_it_is_there() {
        let mut i64 = HashMap::new();
        let mut str_map = HashMap::new();
        let mut bools = HashMap::new();
        let mut vcs_config = InMemory::new(&mut bools, &mut str_map, &mut i64);

        let author = Author::new("Billie Thompson", "billie@example.com", Some("0A46826A"));
        let actual = set_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(
            Some(&"0A46826A".to_string()),
            str_map.get("user.signingkey")
        );
    }

    #[test]
    fn the_first_initial_removes_if_it_is_there_and_not_present() {
        let mut i64 = HashMap::new();
        let mut strs = HashMap::new();
        strs.insert("user.signingkey".into(), "0A46826A".into());
        let mut bools = HashMap::new();

        let mut vcs_config = InMemory::new(&mut bools, &mut strs, &mut i64);

        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let actual = set_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(None, strs.get("user.signingkey"))
    }

    #[test]
    fn multiple_authors_become_coauthors() {
        let mut i64 = HashMap::new();
        let mut strs = HashMap::new();
        let mut bools = HashMap::new();
        let mut vcs_config = InMemory::new(&mut bools, &mut strs, &mut i64);

        let author_1 = Author::new("Billie Thompson", "billie@example.com", None);
        let author_2 = Author::new("Somebody Else", "somebody@example.com", None);
        let author_3 = Author::new("Annie Example", "annie@example.com", None);
        let inputs = vec![&author_1, &author_2, &author_3];

        let actual = set_authors(&mut vcs_config, &inputs, Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(Some(&"Billie Thompson".to_string()), strs.get("user.name"));
        assert_eq!(
            Some(&"billie@example.com".to_string()),
            strs.get("user.email")
        );
        assert_eq!(
            Some(&"Somebody Else".to_string()),
            strs.get("pb.author.coauthors.0.name")
        );
        assert_eq!(
            Some(&"somebody@example.com".to_string()),
            strs.get("pb.author.coauthors.0.email")
        );
        assert_eq!(
            Some(&"Annie Example".to_string()),
            strs.get("pb.author.coauthors.1.name")
        );
        assert_eq!(
            Some(&"annie@example.com".to_string()),
            strs.get("pb.author.coauthors.1.email")
        )
    }

    #[test]
    fn sets_the_expiry_time() {
        let mut i64 = HashMap::new();
        let mut strs = HashMap::new();
        let mut bools = HashMap::new();
        let mut vcs_config = InMemory::new(&mut bools, &mut strs, &mut i64);

        let author = Author::new("Billie Thompson", "billie@example.com", None);
        let actual = set_authors(&mut vcs_config, &[&author], Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());

        let sec59min = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|x| x.add(Duration::from_secs(60 * 59)))
            .map_err(|x| -> Box<dyn Error> { Box::from(x) })
            .map(|x| x.as_secs())
            .and_then(|x| i64::try_from(x).map_err(Box::from))
            .unwrap();

        let sec61min = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|x| x.add(Duration::from_secs(60 * 61)))
            .map_err(|x| -> Box<dyn Error> { Box::from(x) })
            .map(|x| x.as_secs())
            .and_then(|x| i64::try_from(x).map_err(Box::from))
            .unwrap();

        let actual_expire_time = i64.get("pb.author.expires").expect("Failed to read expire");

        assert_eq!(
            true,
            actual_expire_time < &sec61min,
            "Expected less than {}, found {}",
            sec61min,
            actual_expire_time
        );
        assert_eq!(
            true,
            actual_expire_time > &sec59min,
            "Expected more than {}, found {}",
            sec59min,
            actual_expire_time
        );
    }
}
