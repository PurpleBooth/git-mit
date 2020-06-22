use indoc::indoc;
use std::{
    error::Error,
    ops::Add,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use mit_hook_test_helper::assert_output;

#[test]
fn no_issue_number_fail() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output = mit_hook_test_helper::run_hook(&working_dir, "git-mit-relates-to", vec![]);
    let stderr = indoc!(
        "
        error: The following required arguments were not provided:
            <issue-number>

        USAGE:
            git-mit-relates-to <issue-number> --timeout <timeout>

        For more information try --help
        "
    );

    assert_output(&output, "", stderr, false)
}

#[test]
fn set_the_relates_to_information() {
    let working_dir = mit_hook_test_helper::setup_working_dir();
    let output =
        mit_hook_test_helper::run_hook(&working_dir, "git-mit-relates-to", vec!["[#12343567]"]);

    let config = git2::Repository::discover(working_dir)
        .unwrap()
        .config()
        .unwrap()
        .snapshot()
        .unwrap();
    let actual_relates_to = config
        .get_str("mit.relate.to")
        .expect("Failed to read username");

    assert_eq!(actual_relates_to, "[#12343567]");

    let sec59min = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|x| x.add(Duration::from_secs(60 * 59)))
        .unwrap()
        .as_secs();
    let sec61min = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|x| x.add(Duration::from_secs(60 * 61)))
        .unwrap()
        .as_secs();

    let actual_expire_time = config
        .get_str("mit.relate.expires")
        .map_err(Box::from)
        .and_then(|x| -> Result<_, Box<dyn Error>> { u64::from_str(x).map_err(Box::from) })
        .expect("Failed to read expire");

    assert_eq!(
        true,
        actual_expire_time < sec61min,
        "Expected less than {}, found {}",
        sec61min,
        actual_expire_time
    );
    assert_eq!(
        true,
        actual_expire_time > sec59min,
        "Expected more than {}, found {}",
        sec59min,
        actual_expire_time
    );

    assert_output(&output, "", "", true);
}
