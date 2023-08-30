use std::time::SystemTime;

use time::OffsetDateTime;

use crate::mit::AuthorState;

#[test]
fn unwrap_with_value() {
    assert!(AuthorState::Some(true).unwrap());
}

#[test]
#[should_panic = "called `AuthorState::unwrap()` on a `None` value"]
fn unwrap_with_none() {
    assert!(AuthorState::<bool>::None.unwrap());
}

#[test]
#[should_panic = "called `AuthorState::unwrap()` on a `Timeout(1970-01-01 0:00:10.0 +00:00:00)` value"]
fn unwrap_with_timeout() {
    assert!(
        AuthorState::<bool>::Timeout(OffsetDateTime::from_unix_timestamp(10).unwrap()).unwrap()
    );
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
    assert!(!AuthorState::<bool>::Timeout(OffsetDateTime::now_utc()).is_some());
}

#[test]
fn timeout_is_none() {
    assert!(!AuthorState::<bool>::Timeout(OffsetDateTime::now_utc()).is_none());
}

#[test]
fn timeout_is_timeout() {
    assert!(AuthorState::<bool>::Timeout(OffsetDateTime::now_utc()).is_timeout());
}

#[test]
fn is_timeout() {
    assert!(AuthorState::<bool>::Timeout(SystemTime::now().into()).is_timeout());
}
