use chrono::{DateTime, Utc};

#[derive(Debug, Eq, PartialEq)]
pub enum AuthorState<T> {
    Some(T),
    Timeout(DateTime<Utc>),
    None,
}

impl<T> AuthorState<T> {
    pub fn is_none(&self) -> bool {
        matches!(self, AuthorState::<T>::None)
    }

    pub fn is_timeout(&self) -> bool {
        matches!(self, AuthorState::<T>::Timeout(_))
    }

    pub fn is_some(&self) -> bool {
        matches!(self, AuthorState::<T>::Some(_))
    }

    /// Take the value from the state and return it
    ///
    /// # Panics
    ///
    /// Panics if the state is timeout or none
    pub fn unwrap(self) -> T {
        match self {
            AuthorState::Some(value) => value,
            AuthorState::None => panic!("called `AuthorState::unwrap()` on a `None` value"),
            AuthorState::Timeout(value) => panic!(
                "called `AuthorState::unwrap()` on a `Timeout({})` value",
                value
            ),
        }
    }
}

impl<T> From<AuthorState<T>> for Option<T> {
    fn from(values: AuthorState<T>) -> Self {
        match values {
            AuthorState::Some(inner) => Some(inner),
            AuthorState::Timeout(_) | AuthorState::None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use chrono::{DateTime, TimeZone, Utc};

    use crate::mit::AuthorState;

    #[test]
    fn unwrap_with_value() {
        assert!(AuthorState::Some(true).unwrap());
    }

    #[test]
    #[should_panic]
    fn unwrap_with_none() {
        assert!(AuthorState::<bool>::None.unwrap());
    }

    #[test]
    #[should_panic]
    fn unwrap_with_timeout() {
        assert!(AuthorState::<bool>::Timeout(Utc.timestamp(10, 0)).unwrap());
    }

    #[test]
    fn some_is_some() {
        assert!(AuthorState::Some(true).is_some());
    }

    #[test]
    fn some_is_none() {
        assert!(!AuthorState::Some(true).is_none());
    }

    #[test]
    fn some_is_timeout() {
        assert!(!AuthorState::Some(true).is_timeout());
    }

    #[test]
    fn none_is_some() {
        assert!(!AuthorState::<bool>::None.is_some());
    }

    #[test]
    fn none_is_none() {
        assert!(AuthorState::<bool>::None.is_none());
    }

    #[test]
    fn none_is_timeout() {
        assert!(!AuthorState::<bool>::None.is_timeout());
    }

    #[test]
    fn timeout_is_some() {
        assert!(!AuthorState::<bool>::Timeout(DateTime::from(SystemTime::now())).is_some());
    }

    #[test]
    fn timeout_is_none() {
        assert!(!AuthorState::<bool>::Timeout(DateTime::from(SystemTime::now())).is_none());
    }

    #[test]
    fn timeout_is_timeout() {
        assert!(AuthorState::<bool>::Timeout(DateTime::from(SystemTime::now())).is_timeout());
    }

    #[test]
    fn is_timeout() {
        assert!(AuthorState::<bool>::Timeout(SystemTime::now().into()).is_timeout());
    }
}
