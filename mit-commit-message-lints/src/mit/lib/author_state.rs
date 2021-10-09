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
