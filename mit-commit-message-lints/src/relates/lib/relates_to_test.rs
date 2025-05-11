#![allow(clippy::wildcard_imports)]

use crate::relates::RelateTo;

#[test]
fn test_convert_string_to_relate_to() {
    let relate = RelateTo::from("[#12343567]");

    assert_eq!(relate.to(), "[#12343567]");
}
