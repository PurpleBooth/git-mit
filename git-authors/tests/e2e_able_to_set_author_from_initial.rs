use pb_hook_test_helper::assert_output;

use git2::{Config, Repository};
use std::{
    error::Error,
    io::Write,
    ops::Add,
    path::PathBuf,
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tempfile::NamedTempFile;

#[test]
fn no_authors_fail() {
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(&working_dir, "git-authors", vec![]);
    assert_output(
        &output,
        "",
        "error: The following required arguments were not provided:
    <author-initial>...

USAGE:
    git-authors <author-initial>... --config <author-file-path> --timeout <timeout>

For more information try --help
",
        false,
    )
}

#[test]
fn one_initial_sets_that_initial_as_author() {
    let mut author_config =
        tempfile::NamedTempFile::new().expect("Failed to create temporary file");
    let config = r#"
---
bt:
    name: Billie Thompson
    email: billie@example.com
"#;
    write_author_config(&mut author_config, config);
    let working_dir = pb_hook_test_helper::setup_working_dir();
    let output = pb_hook_test_helper::run_hook(
        &working_dir,
        "git-authors",
        vec![
            "-c",
            author_config
                .path()
                .to_str()
                .expect("Failed to convert path to string"),
            "bt",
        ],
    );

    let config = open_config(working_dir);
    let actual_author_name = config
        .get_str("user.name")
        .expect("Failed to read username");
    let actual_author_email = config.get_str("user.email").expect("Failed to read email");

    assert_eq!(actual_author_name, "Billie Thompson");
    assert_eq!(actual_author_email, "billie@example.com");

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
        .get_str("pb.author.expires")
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

fn open_config(working_dir: PathBuf) -> Config {
    let repository = Repository::open(working_dir).expect("Failed to open repository");
    repository
        .config()
        .expect("Failed to open repository config")
        .snapshot()
        .unwrap()
}

fn write_author_config(author_config: &mut NamedTempFile, config: &str) {
    author_config
        .write_all(config.as_bytes())
        .expect("Failed to write to temporary author config");
}
