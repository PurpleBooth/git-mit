use std::str::FromStr;

use super::non_clean_behaviour::BehaviourOption;

#[test]
fn from_str_accepts_lowercase() {
    assert_eq!(
        BehaviourOption::from_str("add-to").unwrap(),
        BehaviourOption::AddTo
    );
    assert_eq!(
        BehaviourOption::from_str("no-change").unwrap(),
        BehaviourOption::NoChange
    );
}

#[test]
fn from_str_rejects_unknown() {
    assert!(BehaviourOption::from_str("unknown").is_err());
}

#[test]
fn from_str_is_case_insensitive_like_value_enum() {
    // clap::ValueEnum accepts any casing; FromStr should too
    assert_eq!(
        BehaviourOption::from_str("Add-To").unwrap(),
        BehaviourOption::AddTo
    );
    assert_eq!(
        BehaviourOption::from_str("ADD-TO").unwrap(),
        BehaviourOption::AddTo
    );
    assert_eq!(
        BehaviourOption::from_str("No-Change").unwrap(),
        BehaviourOption::NoChange
    );
    assert_eq!(
        BehaviourOption::from_str("NO-CHANGE").unwrap(),
        BehaviourOption::NoChange
    );
}

#[test]
fn display_round_trips_through_from_str() {
    for original in [BehaviourOption::AddTo, BehaviourOption::NoChange] {
        let displayed = original.to_string();
        let parsed = BehaviourOption::from_str(&displayed);
        assert_eq!(parsed.unwrap(), original);
    }
}
