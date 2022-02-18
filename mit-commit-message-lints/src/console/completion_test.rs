use std::str::FromStr;

use clap::Command;

use crate::console::completion::{print_completions, Shell};

#[quickcheck]
fn print_completion_bash(shell: Shell) -> bool {
    let mut stdout = Vec::new();

    let mut app =
        Command::new(String::from(env!("CARGO_PKG_NAME"))).about(env!("CARGO_PKG_DESCRIPTION"));
    print_completions(&mut stdout, &mut app, shell);

    let actual = String::from_utf8(stdout).expect("not utf-8");
    !actual.is_empty()
}

#[quickcheck]
fn bi_directional_translation(shell: Shell) -> bool {
    let shell_name: String = shell.into();
    Shell::from_str(&shell_name).expect("expected to be able to convert back to shell") == shell
}

#[test]
fn from_string_bash() {
    let actual: Shell = Shell::from_str("bash").expect("Could not parse shell");
    let expected = Shell::Bash;

    assert_eq!(actual, expected);
}

#[test]
fn into_string_bash() {
    let actual: String = Shell::Bash.into();
    assert_eq!(actual, "bash".to_string());
}

#[test]
fn from_string_fish() {
    let actual: Shell = Shell::from_str("fish").expect("Could not parse shell");
    let expected = Shell::Fish;

    assert_eq!(actual, expected);
}

#[test]
fn into_string_fish() {
    let actual: String = Shell::Fish.into();
    assert_eq!(actual, "fish".to_string());
}

#[test]
fn from_string_zsh() {
    let actual: Shell = Shell::from_str("zsh").expect("Could not parse shell");
    let expected = Shell::Zsh;

    assert_eq!(actual, expected);
}
#[test]
fn into_string_zsh() {
    let actual: String = Shell::Zsh.into();
    assert_eq!(actual, "zsh".to_string());
}

#[test]
fn from_string_elvish() {
    let actual: Shell = Shell::from_str("elvish").expect("Could not parse shell");
    let expected = Shell::Elvish;

    assert_eq!(actual, expected);
}
#[test]
fn into_string_elvish() {
    let actual: String = Shell::Elvish.into();
    assert_eq!(actual, "elvish".to_string());
}

#[test]
fn from_string_powershell() {
    let actual: Shell = Shell::from_str("powershell").expect("Could not parse shell");
    let expected = Shell::PowerShell;

    assert_eq!(actual, expected);
}

#[test]
fn into_string_powershell() {
    let actual: String = Shell::PowerShell.into();
    assert_eq!(actual, "powershell".to_string());
}
