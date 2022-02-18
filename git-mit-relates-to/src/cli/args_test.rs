use mit_commit_message_lints::console::completion::Shell;

use super::args::Args;

#[quickcheck]
fn it_can_give_me_a_shell(shell: Shell) -> bool {
    let app = super::super::app::cli();
    let matches = app.get_matches_from(vec!["binary", "--completion", &String::from(shell)]);
    let actual = Args::from(matches);

    actual.completion().unwrap() == shell
}

#[test]
fn it_defaults_to_none() {
    let app = super::super::app::cli();
    let matches = app.get_matches_from(vec!["binary", "something"]);
    let actual = Args::from(matches);

    assert!(actual.completion().is_none());
}
