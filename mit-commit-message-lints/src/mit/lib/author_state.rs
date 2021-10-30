use time::OffsetDateTime;

/// The state of the authors config
#[derive(Debug, Eq, PartialEq)]
pub enum AuthorState<T> {
    /// Author config all good
    Some(T),

    /// Author config expired
    Timeout(OffsetDateTime),

    /// Author config never set
    None,
}

impl<T> AuthorState<T> {
    /// There has never been author config
    pub const fn is_none(&self) -> bool {
        matches!(self, AuthorState::<T>::None)
    }

    /// The author config has timed out
    pub const fn is_timeout(&self) -> bool {
        matches!(self, AuthorState::<T>::Timeout(_))
    }

    /// The author config looks good
    pub const fn is_some(&self) -> bool {
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
