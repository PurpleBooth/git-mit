use std::{
    convert::TryInto,
    option::Option,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use miette::{IntoDiagnostic, Result, WrapErr};

use crate::{external::Vcs, relates::RelateTo};

const CONFIG_KEY_EXPIRES: &str = "mit.relate.expires";

/// Get the relate-to that are currently defined for this vcs config source
///
/// # Errors
///
/// Will fail if reading or writing from the VCS config fails, or it contains
/// data in an incorrect format
pub fn get_relate_to_configuration(config: &mut dyn Vcs) -> Result<Option<RelateTo<'_>>> {
    let config_value = config.get_i64(CONFIG_KEY_EXPIRES)?;

    match config_value {
        Some(config_value) => {
            let now = now()?;

            if now < Duration::from_secs(config_value.try_into().into_diagnostic()?) {
                let relate_to_config = get_vcs_relate_to(config)?.map(RelateTo::from);

                Ok(relate_to_config)
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

fn now() -> Result<Duration> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .into_diagnostic()
}

#[allow(clippy::maybe_infinite_iter)]
fn get_vcs_relate_to(config: &dyn Vcs) -> Result<Option<&str>> {
    config
        .get_str("mit.relate.to")
        .wrap_err("failed to read relate-to issue")
}
