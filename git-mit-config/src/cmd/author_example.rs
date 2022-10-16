use std::convert::TryInto;

use miette::Result;
use mit_commit_message_lints::{console::style::to_be_piped, mit::Authors};

pub fn run() -> Result<()> {
    let example: String = Authors::example().try_into()?;
    to_be_piped(example.trim());

    Ok(())
}
