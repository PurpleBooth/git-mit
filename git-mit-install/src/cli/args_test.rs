use mit_commit_message_lints::console::completion::Shell;

use super::args::Args;
use crate::cli::args::Scope;

#[test]
fn can_tell_me_if_its_global() {
    let app = super::super::app::app();
    let matches = app.get_matches_from(vec!["binary", "--scope=global"]);
    let actual = Args::from(matches);

    assert_eq!(actual.scope(), Scope::Global);
    assert!(actual.scope().is_global());
}

#[quickcheck]
fn it_can_give_me_a_shell(shell: Shell) -> bool {
    let app = super::super::app::app();
    let matches = app.get_matches_from(vec!["binary", "--completion", &String::from(shell)]);
    let actual = Args::from(matches);

    actual.completion().unwrap() == shell
}

#[test]
fn it_defaults_to_none() {
    let app = super::super::app::app();
    let matches = app.get_matches_from(vec!["binary"]);
    let actual = Args::from(matches);

    assert!(actual.completion().is_none());
}

#[test]
fn can_tell_me_if_its_local() {
    let app = super::super::app::app();
    let matches = app.get_matches_from(vec!["binary"]);
    let actual = Args::from(matches);

    assert_eq!(actual.scope(), Scope::Local);
}
