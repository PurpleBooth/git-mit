use std::{
    convert::TryFrom,
    error,
    time::{Duration, SystemTime},
};

use git2::{Config, ConfigEntries};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Eq, PartialEq)]
pub struct Author {
    name: String,
    email: String,
}

impl Author {
    pub fn new(name: &str, email: &str) -> Author {
        Author {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }
}

pub fn get_author_configuration(config: &Config) -> std::option::Option<Vec<Author>> {
    let time_error_to_false = |err: std::time::SystemTimeError| Box::from(err);
    let right_less_than_left = |pair: (Duration, Duration)| -> bool { pair.0.lt(&pair.1) };
    let i64_into_u64 = |x| u64::try_from(x).map_err(Box::from);

    let config_to_duration_pair =
        |time_since_epoch| -> std::result::Result<(Duration, Duration), Box<dyn Error>> {
            let pair_duration_with_duration =
                |expires_after_time| (time_since_epoch, expires_after_time);
            config
                .get_i64("pb.author.expires")
                .map_err(Box::from)
                .and_then(i64_into_u64)
                .map(Duration::from_secs)
                .map(pair_duration_with_duration)
        };

    match SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(time_error_to_false)
        .and_then(config_to_duration_pair)
        .map(right_less_than_left)
    {
        Ok(true) => {}
        _ => return None,
    }

    let mut authors: Vec<Author> = vec![];

    while let Ok(true) = config_defined(config, &format!("pb.author.coauthors.{}.*", authors.len()))
    {
        let email = match config.get_str(&format!("pb.author.coauthors.{}.email", authors.len())) {
            Ok(email) => email,
            _ => {
                return Some(authors);
            }
        };

        let name = match config.get_str(&format!("pb.author.coauthors.{}.name", authors.len())) {
            Ok(name) => name,
            _ => return Some(authors),
        };

        authors.push(Author::new(name, email))
    }

    Some(authors)
}

fn config_defined(config: &Config, config_key: &str) -> Result<bool> {
    let more_than_1_config_variable = |x: ConfigEntries| x.count() > 1;
    config
        .entries(Some(config_key))
        .map(more_than_1_config_variable)
        .map_err(Box::from)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::wildcard_imports)]

    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn has_an_author() {
        let author = Author::new("The Name", "email@example.com");

        assert_eq!(author.name(), "The Name");
        assert_eq!(author.email(), "email@example.com");
    }
}
