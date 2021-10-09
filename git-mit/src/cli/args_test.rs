use std::{env, ffi::OsString};

use mit_commit_message_lints::{console::completion::Shell, mit::AuthorArgs};
use quickcheck::TestResult;

use crate::cli::{app::app, args::Args};
#[test]
fn can_get_cwd() {
    assert_eq!(Args::cwd().unwrap(), env::current_dir().unwrap());
}

#[quickcheck]
fn timeout_will_be_ok_with_valid_u64(timeout: u64) -> bool {
    Some(timeout)
        == Args::from(app().get_matches_from(vec![
            "git-mit",
            "--timeout",
            &format!("{}", timeout),
            "eg",
        ]))
        .timeout()
        .ok()
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn timeout_will_fail_without_valid_u64(timeout: String) -> TestResult {
    if timeout.parse::<u64>().is_ok() {
        return TestResult::discard();
    }

    if timeout.starts_with('-') {
        return TestResult::discard();
    }

    TestResult::from_bool(
        Args::from(app().get_matches_from(vec!["git-mit", "--timeout", &timeout, "eg"]))
            .timeout()
            .is_err(),
    )
}

#[quickcheck]
fn command_is_none_if_missing(mut cli: Vec<OsString>) -> TestResult {
    if cli.iter().filter(|x| !x.is_empty()).count() == 0 {
        return TestResult::discard();
    }

    let filtered_vec: Vec<_> = cli.clone().into_iter().filter(|x| !x.is_empty()).collect();

    if filtered_vec
        .iter()
        .position(|arg| arg == &OsString::from("--command"))
        .and_then(|x| filtered_vec.iter().filter(|x| !x.is_empty()).nth(x + 1))
        .filter(|x| !x.to_string_lossy().starts_with('-'))
        .map(OsString::from)
        .is_none()
    {
        return TestResult::discard();
    }

    cli.insert(0, OsString::from("eg"));
    cli.insert(0, "git-mit".into());

    TestResult::from_bool(
        Args::from(app().get_matches_from(cli))
            .author_command()
            .is_none(),
    )
}

#[quickcheck]
fn command_is_some_if_present(mut cli: Vec<OsString>, command: OsString) -> TestResult {
    if cli.iter().filter(|x| !x.is_empty()).count() == 0 {
        return TestResult::discard();
    }

    let non_empty_args: Vec<_> = cli.clone().into_iter().filter(|x| !x.is_empty()).collect();

    if non_empty_args
        .iter()
        .position(|arg| arg == &OsString::from("--command"))
        .and_then(|x| non_empty_args.iter().filter(|x| !x.is_empty()).nth(x + 1))
        .and_then(|x| x.to_str())
        .filter(|x| !x.starts_with('-'))
        .map(OsString::from)
        .is_none()
    {
        return TestResult::discard();
    }

    cli.insert(0, command.clone());
    cli.insert(0, OsString::from("--command"));
    cli.insert(0, OsString::from("eg"));
    cli.insert(0, "git-mit".into());

    TestResult::from_bool(
        command.into_string().ok()
            == Args::from(app().get_matches_from(cli))
                .author_command()
                .map(String::from),
    )
}

#[quickcheck]
fn initials_contains_all_initials(mut cli: Vec<OsString>) -> TestResult {
    let expected: Vec<_> = cli
        .iter()
        .filter_map(|x| x.clone().into_string().ok())
        .collect();

    if expected.concat().is_empty() || expected.iter().any(|x| x.starts_with('-')) {
        return TestResult::discard();
    }

    cli.insert(0, OsString::from("git-mit"));

    let args = Args::from(app().get_matches_from(cli.clone()));
    let actual: Vec<String> = args
        .initials()
        .unwrap()
        .into_iter()
        .map(String::from)
        .collect();
    TestResult::from_bool(expected == actual)
}

#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn config_file_missing_defaults(mut cli: Vec<OsString>) -> TestResult {
    if cli.clone().iter().filter(|x| !x.is_empty()).count() == 0 {
        return TestResult::discard();
    }

    let filtered_vec: Vec<_> = cli.clone().into_iter().filter(|x| !x.is_empty()).collect();

    if filtered_vec
        .iter()
        .position(|arg| arg == &OsString::from("--config"))
        .and_then(|x| filtered_vec.iter().filter(|x| !x.is_empty()).nth(x + 1))
        .filter(|x| !x.to_string_lossy().starts_with('-'))
        .map(OsString::from)
        .is_none()
    {
        return TestResult::discard();
    }

    cli.insert(0, "eg".into());
    cli.insert(0, "git-mit".into());

    TestResult::from_bool(
        Some("$HOME/.config/git-mit/mit.toml")
            == Args::from(app().get_matches_from(cli)).author_file(),
    )
}
#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn config_file_defined_returns(file: OsString) -> TestResult {
    if file.is_empty() || file.to_str().map(|x| x.starts_with('-')).is_some() {
        return TestResult::discard();
    }

    let args = vec!["git-mit".into(), "-c".into(), file.clone(), "eg".into()];

    TestResult::from_bool(file.to_str() == Args::from(app().get_matches_from(args)).author_file())
}
#[allow(clippy::needless_pass_by_value)]
#[quickcheck]
fn completion_with_defined_value_returns(shell: Shell) -> TestResult {
    let args = vec!["git-mit".into(), "--completion".into(), String::from(shell)];

    TestResult::from_bool(
        shell
            == Args::from(app().get_matches_from(args))
                .completion()
                .unwrap(),
    )
}

#[test]
fn completion_is_none_by_default() {
    let args: Vec<String> = vec!["git-mit".into(), "bt".into()];

    assert!(Args::from(app().get_matches_from(args))
        .completion()
        .is_none(),);
}
