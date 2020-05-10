use std::{
    convert::TryFrom,
    error,
    error::Error,
    time::{Duration, SystemTime},
};

use git2::{Config, ConfigEntries};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Eq, PartialEq)]
pub struct Author {
    name: String,
    email: String,
}

impl Author {
    #[must_use]
    pub fn new(name: &str, email: &str) -> Author {
        Author {
            name: name.to_string(),
            email: email.to_string(),
        }
    }

    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub fn email(&self) -> String {
        self.email.clone()
    }
}

#[must_use]
pub fn get_author_configuration(config: &Config) -> std::option::Option<Vec<Author>> {
    let right_less_than_left = |pair: (Duration, Duration)| -> bool { pair.0.lt(&pair.1) };
    let i64_into_u64 = |x| u64::try_from(x).map_err(Box::<dyn Error>::from);
    let left_time_right_duration = |expires_after_time| {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(Box::from)
            .map(|time_since_epoch| (time_since_epoch, expires_after_time))
    };
    let is_true = |x: &bool| true.eq(&x);
    let replace_with_coauthors = |_| defined_coauthors(config);

    config
        .get_i64("pb.author.expires")
        .map_err(Box::from)
        .and_then(i64_into_u64)
        .map(Duration::from_secs)
        .and_then(left_time_right_duration)
        .map(right_less_than_left)
        .ok()
        .filter(is_true)
        .map(replace_with_coauthors)
}

fn defined_coauthors(config: &Config) -> Vec<Author> {
    let mut authors: Vec<Author> = vec![];

    while let Ok(true) = config_defined(config, &format!("pb.author.coauthors.{}.*", authors.len()))
    {
        let email = if let Ok(email) =
            config.get_str(&format!("pb.author.coauthors.{}.email", authors.len()))
        {
            email
        } else {
            return authors;
        };

        let name = match config.get_str(&format!("pb.author.coauthors.{}.name", authors.len())) {
            Ok(name) => name,
            _ => return authors,
        };

        authors.push(Author::new(name, email))
    }

    authors
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
