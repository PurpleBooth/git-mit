use std::fmt;
use std::str::FromStr;

use super::Error;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Trailer {
    key: String,
    value: String,
}

impl Trailer {
    #[must_use]
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }

    #[must_use]
    pub fn has_key(&self, key: &str) -> bool {
        key == self.key
    }
}

impl FromStr for Trailer {
    type Err = Error;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.contains(':') {
            let parts: Vec<&str> = string.splitn(2, ':').collect();
            Ok(Trailer {
                key: parts[0].trim().into(),
                value: parts[1].trim().into(),
            })
        } else {
            Err(Error::FailedToParseTrailer {
                string: string.into(),
            })
        }
    }
}

impl fmt::Display for Trailer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.key, self.value)
    }
}

#[cfg(test)]
mod test_trailer {
    use super::{Error, FromStr, Trailer};

    #[test]
    fn from_str_with_bad_trailer() {
        assert_eq!(
            Err(Error::FailedToParseTrailer {
                string: "no colon here".into(),
            }),
            Trailer::from_str("no colon here")
        )
    }

    #[test]
    fn from_str_with_good_trailer() {
        assert_eq!(
            Ok(Trailer::new("Key", "Value")),
            Trailer::from_str("Key:Value")
        )
    }

    #[test]
    fn from_str_with_good_trailer_with_whitespace() {
        assert_eq!(
            Ok(Trailer::new("Key Part", "Value Part")),
            Trailer::from_str(" Key Part : Value Part ")
        )
    }

    #[test]
    fn to_str_returns_formatted_trailer() {
        assert_eq!(
            "Trailer: Value",
            Trailer::new("Trailer", "Value").to_string()
        )
    }

    #[test]
    fn has_key_returns_false_if_key_does_not_match() {
        assert_eq!(false, Trailer::new("key", "value").has_key("not-key"))
    }

    #[test]
    fn has_key_returns_true_if_key_does_match() {
        assert_eq!(true, Trailer::new("key", "value").has_key("key"))
    }
}
