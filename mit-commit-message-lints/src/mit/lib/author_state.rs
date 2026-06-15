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
        matches!(self, Self::None)
    }

    /// The author config has timed out
    pub const fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }

    /// The author config looks good
    pub const fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    /// Take the value from the state and return it
    ///
    /// # Panics
    ///
    /// Panics if the state is timeout or none
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(value) => value,
            Self::None => panic!("called `AuthorState::unwrap()` on a `None` value"),
            Self::Timeout(value) => panic!(
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
mod tests {
    use std::time::SystemTime;

    use time::OffsetDateTime;

    use crate::mit::AuthorState;

    #[test]
    fn test_unwrap_some_succeeds() {
        assert!(
            AuthorState::Some(true).unwrap(),
            "Expected unwrap on Some state to return the inner value"
        );
    }

    #[test]
    #[should_panic = "called `AuthorState::unwrap()` on a `None` value"]
    fn test_unwrap_none_panics() {
        assert!(
            AuthorState::<bool>::None.unwrap(),
            "Expected unwrap on None state to panic"
        );
    }

    #[test]
    #[should_panic = "called `AuthorState::unwrap()` on a `Timeout(1970-01-01 0:00:10.0 +00:00:00)` value"]
    fn test_unwrap_timeout_panics() {
        assert!(
            AuthorState::<bool>::Timeout(OffsetDateTime::from_unix_timestamp(10).unwrap()).unwrap(),
            "Expected unwrap on Timeout state to panic"
        );
    }

    #[test]
    fn test_some_state_is_some() {
        assert!(
            AuthorState::Some(true).is_some(),
            "Expected Some state to report is_some as true"
        );
    }

    #[test]
    fn test_some_state_is_not_none() {
        assert!(
            !AuthorState::Some(true).is_none(),
            "Expected Some state to report is_none as false"
        );
    }

    #[test]
    fn test_some_state_is_not_timeout() {
        assert!(
            !AuthorState::Some(true).is_timeout(),
            "Expected Some state to report is_timeout as false"
        );
    }

    #[test]
    fn test_none_state_is_not_some() {
        assert!(
            !AuthorState::<bool>::None.is_some(),
            "Expected None state to report is_some as false"
        );
    }

    #[test]
    fn test_none_state_is_none() {
        assert!(
            AuthorState::<bool>::None.is_none(),
            "Expected None state to report is_none as true"
        );
    }

    #[test]
    fn test_none_state_is_not_timeout() {
        assert!(
            !AuthorState::<bool>::None.is_timeout(),
            "Expected None state to report is_timeout as false"
        );
    }

    #[test]
    fn test_timeout_state_is_not_some() {
        assert!(
            !AuthorState::<bool>::Timeout(OffsetDateTime::now_utc()).is_some(),
            "Expected Timeout state to report is_some as false"
        );
    }

    #[test]
    fn test_timeout_state_is_not_none() {
        assert!(
            !AuthorState::<bool>::Timeout(OffsetDateTime::now_utc()).is_none(),
            "Expected Timeout state to report is_none as false"
        );
    }

    #[test]
    fn test_timeout_state_recognized() {
        assert!(
            AuthorState::<bool>::Timeout(OffsetDateTime::now_utc()).is_timeout(),
            "Expected Timeout state to report is_timeout as true"
        );
    }

    #[test]
    fn test_system_time_timeout_recognition() {
        assert!(
            AuthorState::<bool>::Timeout(SystemTime::now().into()).is_timeout(),
            "Expected a Timeout constructed from SystemTime to be recognized as timeout"
        );
    }
}
