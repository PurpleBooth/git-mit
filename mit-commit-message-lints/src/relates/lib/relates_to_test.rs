#![allow(clippy::wildcard_imports)]

use crate::relates::RelateTo;

#[test]
fn has_a_relate_to_string() {
    let relate = RelateTo::new("[#12343567]");

    assert_eq!(relate.to(), "[#12343567]");
}
