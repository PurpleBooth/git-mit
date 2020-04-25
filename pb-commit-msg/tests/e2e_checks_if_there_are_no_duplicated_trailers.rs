use std::io::Write;
use std::process::Command;
use std::str;
use tempfile::NamedTempFile;

#[test]
fn duplicated_trailers_cause_errors() {
    let input = r#"An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>
"#;
    let mut tmpfile = NamedTempFile::new().unwrap();
    write!(tmpfile, "{}", input).unwrap();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg(tmpfile.path().to_str().unwrap())
        .output()
        .expect("failed to execute process");

    assert!(!&output.status.success());
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(
        stdout.is_empty(),
        "Expected stdout to be empty, instead it contained \"{}\"",
        stdout
    );

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert_eq!(
        stderr.to_string(),
        r#"
An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
Signed-off-by: Billie Thompson <email@example.com>


Your commit cannot have the same name duplicated in the \"Signed-off-by\" field

"#
    );
}

#[test]
fn a_valid_commit_is_fine() {
    let input = r#"An example commit

This is an example commit with duplicate trailers

Signed-off-by: Billie Thompson <email@example.com>
"#;
    let mut tmpfile = NamedTempFile::new().unwrap();
    write!(tmpfile, "{}", input).unwrap();

    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .arg(tmpfile.path().to_str().unwrap())
        .output()
        .expect("failed to execute process");
    let stdout = str::from_utf8(&output.stdout).unwrap();
    assert!(
        stdout.is_empty(),
        "Expected stdout to be empty, instead it contained \"{}\"",
        stdout
    );

    let stderr = str::from_utf8(&output.stderr).unwrap();
    assert!(
        stderr.is_empty(),
        "Expected stderr to be empty, instead it contained \"{}\"",
        stderr
    );

    assert!(
        &output.status.success(),
        "Expected status to be successful, instead it was {}",
        &output.status.code().unwrap()
    );
}
