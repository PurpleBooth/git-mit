use std::{
    convert::TryFrom,
    error,
    time::{Duration, SystemTime},
};

use git2::Config;

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
    match SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_err| false)
        .and_then(
            |time_since_epoch| -> std::result::Result<(Duration, Duration), bool> {
                config
                    .get_i64("pb.author.expires")
                    .map_err(|_err| false)
                    .and_then(|x| u64::try_from(x).map_err(|_x| false))
                    .map(Duration::from_secs)
                    .map(|expires_after_time| (time_since_epoch, expires_after_time))
                    .map_err(|_err| -> bool { false })
            },
        )
        .map(|(time_since_epoch, expires_after_time)| time_since_epoch.lt(&expires_after_time))
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
    config
        .entries(Some(config_key))
        .map(|x| x.count() > 1)
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
