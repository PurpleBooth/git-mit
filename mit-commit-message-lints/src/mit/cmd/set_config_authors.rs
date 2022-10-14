use miette::Result;

use crate::{external::Vcs, mit::Author};
/// # Errors
///
/// On write failure
pub fn set_config_authors(store: &mut dyn Vcs, initial: &str, author: &Author<'_>) -> Result<()> {
    store.set_str(
        &format!("mit.author.config.{initial}.email"),
        author.email(),
    )?;
    store.set_str(&format!("mit.author.config.{initial}.name"), author.name())?;

    if let Some(signingkey) = author.signingkey() {
        store.set_str(
            &format!("mit.author.config.{initial}.signingkey"),
            signingkey,
        )?;
    }

    Ok(())
}
