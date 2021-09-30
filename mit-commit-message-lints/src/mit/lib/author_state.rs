#[derive(Debug, Eq, PartialEq)]
pub enum AuthorState<T> {
    Some(T),
    Timeout(i64),
    None,
}

impl<T> AuthorState<T> {
    pub fn is_some(&self) -> bool {
        matches!(self, AuthorState::<T>::Some(_))
    }
}

impl<T> AuthorState<T> {
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
        assert!(AuthorState::<bool>::Timeout(10_i64).unwrap());
    }

    #[test]
    fn is_some() {
        assert!(AuthorState::Some(true).is_some());
    }
}
