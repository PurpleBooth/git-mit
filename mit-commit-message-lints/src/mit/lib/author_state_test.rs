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
