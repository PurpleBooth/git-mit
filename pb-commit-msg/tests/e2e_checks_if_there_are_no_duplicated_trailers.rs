use std::{io::Write, process::Command};

use indoc::indoc;
use tempfile::NamedTempFile;

use pb_hook_test_helper::{assert_output, setup_working_dir};

#[test]
fn duplicated_trailer() {
    let input = indoc!(
        "
        An example commit

        This is an example commit with duplicate trailers

        Signed-off-by: Billie Thompson <email@example.com>
        Signed-off-by: Billie Thompson <email@example.com>
        "
    );

    let working_dir = setup_working_dir();

    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", input).unwrap();

    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-commit-msg",
        vec![commit_path.path().to_str().unwrap()],
    );

    assert_output(
        &output,
        "",
        indoc!(
            "An example commit

            This is an example commit with duplicate trailers

            Signed-off-by: Billie Thompson <email@example.com>
            Signed-off-by: Billie Thompson <email@example.com>


            ---

            Your commit cannot have the same name duplicated in the \"Signed-off-by\" field

            You can fix this by removing the duplicated field when you commit again

            "
        ),
        false,
    )
}

#[test]
fn valid_commit() {
    let input = indoc!(
        "
        An example commit

        This is an example commit with duplicate trailers

        Signed-off-by: Billie Thompson <email@example.com>
        "
    );

    let working_dir = setup_working_dir();

    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", input).unwrap();

    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-commit-msg",
        vec![commit_path.path().to_str().unwrap()],
    );

    assert_output(&output, "", r#""#, true)
}

#[test]
fn disabled() {
    let input = indoc!(
        "
        An example commit

        This is an example commit with duplicate trailers

        Signed-off-by: Billie Thompson <email@example.com>
        Signed-off-by: Billie Thompson <email@example.com>
        "
    );
    let working_dir = setup_working_dir();
    Command::new("git")
        .current_dir(&working_dir)
        .arg("config")
        .arg("--local")
        .arg("pb.lint.duplicated-trailers")
        .arg("false")
        .output()
        .expect("failed to execute process");

    let mut commit_path = NamedTempFile::new().unwrap();
    write!(commit_path, "{}", input).unwrap();

    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "pb-commit-msg",
        vec![commit_path.path().to_str().unwrap()],
    );
    assert_output(&output, "", r#""#, true)
}
