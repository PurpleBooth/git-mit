use std::str::FromStr;

use super::rotation_option::RotationOption;

#[test]
fn from_str_accepts_lowercase() {
    assert_eq!(
        RotationOption::from_str("round-robin").unwrap(),
        RotationOption::RoundRobin
    );
}

#[test]
fn from_str_rejects_unknown() {
    assert!(RotationOption::from_str("unknown").is_err());
}

#[test]
fn from_str_is_case_insensitive_like_value_enum() {
    assert_eq!(
        RotationOption::from_str("Round-Robin").unwrap(),
        RotationOption::RoundRobin
    );
    assert_eq!(
        RotationOption::from_str("ROUND-ROBIN").unwrap(),
        RotationOption::RoundRobin
    );
}

#[test]
fn display_round_trips_through_from_str() {
    let original = RotationOption::RoundRobin;
    let displayed = original.to_string();
    let parsed = RotationOption::from_str(&displayed);
    assert_eq!(parsed.unwrap(), original);
}
