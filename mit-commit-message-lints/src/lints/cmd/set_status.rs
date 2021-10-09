use miette::Result;
use mit_lint::Lints;

use crate::external::Vcs;
/// # Errors
///
/// Errors if writing to the VCS config fails
pub fn set_status(lints: Lints, vcs: &mut dyn Vcs, status: bool) -> Result<()> {
    lints
        .config_keys()
        .into_iter()
        .try_for_each(|lint| vcs.set_str(&lint, &status.to_string()))?;
    Ok(())
}
