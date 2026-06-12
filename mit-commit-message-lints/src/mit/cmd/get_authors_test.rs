use std::io::Write;

use crate::mit::{get_authors, GenericArgs};

#[test]
#[cfg(unix)]
fn unreadable_author_file_returns_error() {
    use std::os::unix::fs::PermissionsExt;

    let mut temp_file = std::env::temp_dir();
    temp_file.push(format!("unreadable_mit_test_{}.toml", std::process::id()));

    let _ = std::fs::remove_file(&temp_file);

    {
        let mut file = std::fs::File::create(&temp_file).unwrap();
        file.write_all(b"[authors]").unwrap();
    }

    let mut permissions = std::fs::metadata(&temp_file).unwrap().permissions();
    permissions.set_mode(0o000);
    std::fs::set_permissions(&temp_file, permissions).unwrap();

    let args = GenericArgs {
        author_command: None,
        author_file: Some(temp_file.to_str().unwrap()),
    };

    let result = get_authors(&args);
    assert!(
        result.is_err(),
        "expected an IO error when the author file is unreadable, but got Ok"
    );

    // Cleanup
    let mut permissions = std::fs::metadata(&temp_file).unwrap().permissions();
    permissions.set_mode(0o644);
    std::fs::set_permissions(&temp_file, permissions).unwrap();
    let _ = std::fs::remove_file(&temp_file);
}
