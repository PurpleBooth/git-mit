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
fn from_str_accepts_random() {
    assert_eq!(
        RotationOption::from_str("random").unwrap(),
        RotationOption::Random
    );
}

#[test]
fn from_str_accepts_random_case_insensitive() {
    assert_eq!(
        RotationOption::from_str("Random").unwrap(),
        RotationOption::Random
    );
    assert_eq!(
        RotationOption::from_str("RANDOM").unwrap(),
        RotationOption::Random
    );
}

#[test]
fn display_round_trips_through_from_str() {
    for original in [
        RotationOption::Off,
        RotationOption::RoundRobin,
        RotationOption::Random,
    ] {
        let displayed = original.to_string();
        let parsed = RotationOption::from_str(&displayed);
        assert_eq!(parsed.unwrap(), original);
    }
}

#[test]
fn display_random_round_trips() {
    let original = RotationOption::Random;
    let displayed = original.to_string();
    let parsed = RotationOption::from_str(&displayed);
    assert_eq!(parsed.unwrap(), original);
}
