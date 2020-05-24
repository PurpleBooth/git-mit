use std::{
    convert::TryFrom,
    error::Error,
    time::{Duration, SystemTime},
};

use crate::{author::entities::Author, external::vcs::Vcs};
use std::{ops::Add, time::UNIX_EPOCH};

const CONFIG_KEY_EXPIRES: &str = "pb.author.expires";

#[must_use]
pub fn get_coauthor_configuration(config: &dyn Vcs) -> std::option::Option<Vec<Author>> {
    config
        .get_i64(CONFIG_KEY_EXPIRES)
        .ok_or_else(|| "No author expiry date".into())
        .and_then(i64_into_u64)
        .map(Duration::from_secs)
        .and_then(time_and_now)
        .map(is_after)
        .ok()
        .filter(bool::clone)
        .map(partial!(replace_with_coauthors => _, config))
}

fn time_and_now(expires_after_time: Duration) -> Result<(Duration, Duration), Box<dyn Error>> {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(Box::from)
        .map(partial!(duration_tuple => expires_after_time, _))
}

fn duration_tuple(
    expires_after_time: Duration,
    time_since_epoch: Duration,
) -> (Duration, Duration) {
    (time_since_epoch, expires_after_time)
}

fn replace_with_coauthors(_: bool, config: &dyn Vcs) -> Vec<Author> {
    defined_coauthors(config)
}

fn i64_into_u64(x: i64) -> Result<u64, Box<dyn Error>> {
    u64::try_from(x).map_err(Box::<dyn Error>::from)
}

fn u64_into_i64(x: u64) -> Result<i64, Box<dyn Error>> {
    i64::try_from(x).map_err(Box::<dyn Error>::from)
}

fn is_after((point, comparison): (Duration, Duration)) -> bool {
    point.lt(&comparison)
}

fn defined_coauthors(config: &dyn Vcs) -> Vec<Author> {
    get_config_names(config)
        .iter()
        .zip(get_config_emails(config))
        .filter_map(tuple_to_author)
        .collect()
}

fn tuple_to_author(a: (&Option<&str>, Option<&str>)) -> Option<Author> {
    match a {
        (Some(name), Some(email)) => Some(Author::new(name, email)),
        _ => None,
    }
}

fn get_config_names(config: &dyn Vcs) -> Vec<Option<&str>> {
    get_config_values(config, "name")
}

fn get_config_emails(config: &dyn Vcs) -> Vec<Option<&str>> {
    get_config_values(config, "email")
}

#[allow(clippy::maybe_infinite_iter)]
fn get_config_values<'a>(config: &'a dyn Vcs, key: &str) -> Vec<Option<&'a str>> {
    (0..)
        .take_while(|x| config_id_exists(config, *x))
        .map(partial!(get_from_config => config, key, _))
        .collect()
}

fn get_from_config<'a>(config: &'a dyn Vcs, key: &str, x: i32) -> Option<&'a str> {
    config.get_str(&format!("pb.author.coauthors.{}.{}", x, key))
}

fn config_id_exists(config: &dyn Vcs, id: i32) -> bool {
    read_email_from_config(config, id).is_some()
}

fn read_email_from_config(config: &'_ dyn Vcs, id: i32) -> Option<&'_ str> {
    config.get_str(&format!("pb.author.coauthors.{}.email", id))
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

        let bools: HashMap<String, bool> = HashMap::new();
        let mut strings: HashMap<String, String> = HashMap::new();
        let mut i64_configs = HashMap::new();
        i64_configs.insert("pb.author.expires".into(), now_minus_10);
        let git2_config = InMemory::new(&bools, &mut strings, &mut i64_configs);

        let actual = get_coauthor_configuration(&git2_config);
        let expected = None;
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn there_is_a_config_if_the_config_has_not_expired() {
        let now_plus_10_seconds = epoch_with_offset(add_10_seconds);

        let mut i64_configs = HashMap::new();
        i64_configs.insert("pb.author.expires".into(), now_plus_10_seconds);
        let bools = HashMap::new();
        let mut strings = HashMap::new();
        let mut i64s = i64_configs;
        let git2_config = InMemory::new(&bools, &mut strings, &mut i64s);

        let actual = get_coauthor_configuration(&git2_config);

        let expected: Option<Vec<Author>> = Some(vec![]);
        assert_eq!(
            expected, actual,
            "Expected the author config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn we_get_author_config_back_if_there_is_any() {
        let now_plus_10_seconds = epoch_with_offset(add_10_seconds);

        let mut i64_configs = HashMap::new();
        i64_configs.insert("pb.author.expires".into(), now_plus_10_seconds);
        let mut str_configs = HashMap::new();
        str_configs.insert(
            "pb.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        str_configs.insert("pb.author.coauthors.0.name".into(), "Annie Example".into());
        let bools = HashMap::new();
        let git2_config = InMemory::new(&bools, &mut str_configs, &mut i64_configs);

        let actual = get_coauthor_configuration(&git2_config);

        let expected = Some(vec![Author::new("Annie Example", "annie@example.com")]);
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
        let now_plus_10_seconds = epoch_with_offset(add_10_seconds);
        let mut i64_configs = HashMap::new();
        i64_configs.insert("pb.author.expires".into(), now_plus_10_seconds);
        let mut str_configs = HashMap::new();
        str_configs.insert(
            "pb.author.coauthors.0.email".into(),
            "annie@example.com".into(),
        );
        str_configs.insert("pb.author.coauthors.0.name".into(), "Annie Example".into());
        str_configs.insert(
            "pb.author.coauthors.1.email".into(),
            "joe@example.com".into(),
        );
        str_configs.insert("pb.author.coauthors.1.name".into(), "Joe Bloggs".into());
        let bools = HashMap::new();
        let git2_config = InMemory::new(&bools, &mut str_configs, &mut i64_configs);

        let actual = get_coauthor_configuration(&git2_config);
        let expected = Some(vec![
            Author::new("Annie Example", "annie@example.com"),
            Author::new("Joe Bloggs", "joe@example.com"),
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
    let authors = authors
        .first()
        .ok_or_else(|| -> Box<dyn Error> { "You need at least one author".into() })?;

    config.set_str("user.name", &authors.name())?;
    config.set_str("user.email", &authors.email())?;

    let expire_time: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(Box::from)
        .map(|x| x.add(expires_in))
        .map(|x| x.as_secs())
        .and_then(u64_into_i64)
        .unwrap();
    config.set_i64(CONFIG_KEY_EXPIRES, expire_time)?;

    Ok(())
}

#[cfg(test)]
mod tests_can_set_author_details {
    use crate::{
        author::{entities::Author, vcs::set_authors},
        external::vcs::InMemory,
    };
    use std::{
        collections::HashMap,
        convert::TryFrom,
        error::Error,
        ops::Add,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn we_get_author_config_back_if_there_is_any() {
        let mut i64_map = HashMap::new();
        let mut str_map = HashMap::new();

        let bools = HashMap::new();
        let mut vcs_config = InMemory::new(&bools, &mut str_map, &mut i64_map);

        let author = Author::new("Billie Thompson", "billie@example.com");
        let input = vec![&author];
        let actual = set_authors(&mut vcs_config, &input, Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(
            Some(&"Billie Thompson".to_string()),
            str_map.get("user.name")
        );
        assert_eq!(
            Some(&"billie@example.com".to_string()),
            str_map.get("user.email")
        )
    }
    #[test]
    fn sets_the_expiry_time() {
        let mut i64_map = HashMap::new();
        let mut str_map = HashMap::new();

        let bools = HashMap::new();
        let mut vcs_config = InMemory::new(&bools, &mut str_map, &mut i64_map);

        let author = Author::new("Billie Thompson", "billie@example.com");
        let input = vec![&author];
        let _actual = set_authors(&mut vcs_config, &input, Duration::from_secs(60 * 60));

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

        let actual_expire_time = i64_map
            .get("pb.author.expires")
            .expect("Failed to read expire");

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
