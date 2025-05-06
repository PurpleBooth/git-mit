use std::{convert::TryInto, time::Duration};

use miette::{miette, Result, WrapErr};
use time::OffsetDateTime;

use crate::{external::Vcs, relates::RelateTo};
const CONFIG_KEY_EXPIRES: &str = "mit.relate.expires";

/// # Errors
///
/// This errors if writing to the git mit file fails for some reason. Those
/// reasons will be specific to VCS implementation
pub fn set_relates_to(
    config: &mut dyn Vcs,
    relates: &RelateTo<'_>,
    expires_in: Duration,
) -> Result<()> {
    set_vcs_relates_to(config, relates)?;
    set_vcs_expires_time(config, expires_in)?;

    Ok(())
}

fn set_vcs_relates_to(config: &mut dyn Vcs, relates: &RelateTo<'_>) -> Result<()> {
    config.set_str("mit.relate.to", relates.to())?;
    Ok(())
}

fn set_vcs_expires_time(config: &mut dyn Vcs, expires_in: Duration) -> Result<()> {
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let expires_in_secs: i64 = expires_in
        .as_secs()
        .try_into()
        .map_err(|_| miette!("Expiration time exceeds maximum supported value"))?;

    let expiry_time = now
        .checked_add(expires_in_secs)
        .ok_or_else(|| miette!("Expiration time overflow"))?;

    config
        .set_i64(CONFIG_KEY_EXPIRES, expiry_time)
        .wrap_err("failed to update the expiry time mit-relates-to")
}
