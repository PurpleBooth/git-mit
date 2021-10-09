use miette::Result;

use crate::{external::Vcs, mit::Author};
/// # Errors
///
/// On write failure
pub fn set_config_authors(store: &mut dyn Vcs, initial: &str, author: &Author) -> Result<()> {
    store.set_str(
        &format!("mit.author.config.{}.email", initial),
        &author.email(),
    )?;
    store.set_str(
        &format!("mit.author.config.{}.name", initial),
        &author.name(),
    )?;

    if let Some(signingkey) = author.signingkey() {
        store.set_str(
            &format!("mit.author.config.{}.signingkey", initial),
            &signingkey,
        )?;
    }

    Ok(())
}
