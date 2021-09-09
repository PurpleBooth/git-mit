use std::{convert::TryFrom, env};

use cli::{app, args::Args};
use mit_commit_message_lints::{
    external::Git2,
    relates::{entities::RelateTo, vcs::set_relates_to},
};

mod cli;
mod errors;

fn main() -> Result<(), errors::GitRelatesTo> {
    let args: Args = app::app().get_matches().into();

    let relates_to = args.issue_number()?;

    let current_dir = env::current_dir()?;
    let mut vcs = Git2::try_from(current_dir)?;
    set_relates_to(&mut vcs, &RelateTo::new(relates_to), args.timeout()?)?;

    Ok(())
}
