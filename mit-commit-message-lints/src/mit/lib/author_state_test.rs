use std::time::SystemTime;

use time::OffsetDateTime;

use crate::mit::AuthorState;

#[test]
fn test_unwrap_some_succeeds() {
    assert!(AuthorState::Some(true).unwrap());
}

#[test]
#[should_panic = "called `AuthorState::unwrap()` on a `None` value"]
fn test_unwrap_none_panics() {
    assert!(AuthorState::<bool>::None.unwrap());
}

#[test]
#[should_panic = "called `AuthorState::unwrap()` on a `Timeout(1970-01-01 0:00:10.0 +00:00:00)` value"]
fn test_unwrap_timeout_panics() {
    assert!(
        AuthorState::<bool>::Timeout(OffsetDateTime::from_unix_timestamp(10).unwrap()).unwrap()
    );
}

#[test]
fn test_some_state_is_some() {
    assert!(AuthorState::Some(true).is_some());
}

#[test]
fn test_some_state_is_not_none() {
    assert!(!AuthorState::Some(true).is_none());
}

#[test]
fn test_some_state_is_not_timeout() {
    assert!(!AuthorState::Some(true).is_timeout());
}

#[test]
fn test_none_state_is_not_some() {
    assert!(!AuthorState::<bool>::None.is_some());
}

#[test]
fn test_none_state_is_none() {
    assert!(AuthorState::<bool>::None.is_none());
}

#[test]
fn test_none_state_is_not_timeout() {
    assert!(!AuthorState::<bool>::None.is_timeout());
}

#[test]
fn test_timeout_state_is_not_some() {
    assert!(!AuthorState::<bool>::Timeout(OffsetDateTime::now_utc()).is_some());
}

#[test]
fn test_timeout_state_is_not_none() {
    assert!(!AuthorState::<bool>::Timeout(OffsetDateTime::now_utc()).is_none());
}

#[test]
fn test_timeout_state_recognized() {
    assert!(AuthorState::<bool>::Timeout(OffsetDateTime::now_utc()).is_timeout());
}

#[test]
fn test_system_time_timeout_recognition() {
    assert!(AuthorState::<bool>::Timeout(SystemTime::now().into()).is_timeout());
}
