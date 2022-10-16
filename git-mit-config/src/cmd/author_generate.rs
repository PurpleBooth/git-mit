use std::convert::TryFrom;

use miette::Result;
use mit_commit_message_lints::{
    console::style::author_table,
    external::Git2,
    mit::{get_authors, Authors, GenericArgs},
};

use crate::current_dir;

pub fn run_available(config: &str, exec: &Option<String>) -> Result<()> {
    let args = GenericArgs {
        author_command: exec.as_ref().map(|x| x as _),
        author_file: Some(config),
    };
    let file_authors = get_authors(&args)?;
    let git_config = current_dir().and_then(Git2::try_from)?;
    let authors = file_authors.merge(&Authors::try_from(&git_config)?);

    let output: String = author_table(&authors);

    mit_commit_message_lints::console::style::to_be_piped(&output);
    Ok(())
}

pub fn run_generate(config: &str, exec: &Option<String>) -> Result<()> {
    let args = GenericArgs {
        author_command: exec.as_ref().map(|x| x as _),
        author_file: Some(config),
    };
    let file_authors = get_authors(&args)?;
    let git_config = current_dir().and_then(Git2::try_from)?;
    let authors = file_authors.merge(&Authors::try_from(&git_config)?);

    let output: String = to_toml(authors)?;

    mit_commit_message_lints::console::style::to_be_piped(&output);
    Ok(())
}

fn to_toml(authors: Authors<'_>) -> Result<String> {
    Ok(String::try_from(authors)?.trim().to_string())
}
