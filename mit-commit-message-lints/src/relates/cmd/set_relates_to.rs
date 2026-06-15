use std::{convert::TryInto, time::Duration};

use miette::{miette, Result, WrapErr};
use time::OffsetDateTime;

use crate::{external::Vcs, relates::RelateTo};
const CONFIG_KEY_EXPIRES: &str = "mit.relate.expires";

/// # Errors
///
/// If writing to the git mit file fails for some reason. (specific to VCS implementation)
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

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        convert::TryFrom,
        error::Error,
        ops::Add,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use crate::{
        external::InMemory,
        relates::{set_relates_to, RelateTo},
    };

    #[test]
    fn the_first_initial_becomes_the_relates() {
        let mut buffer = BTreeMap::new();

        let mut vcs_config = InMemory::new(&mut buffer);

        let relates_to = RelateTo::from("[#12345678]");
        let actual = set_relates_to(&mut vcs_config, &relates_to, Duration::from_hours(1));

        actual.unwrap();
        assert_eq!(
            Some(&"[#12345678]".to_string()),
            buffer.get("mit.relate.to")
        );
    }

    #[test]
    fn sets_the_expiry_time() {
        let mut buffer = BTreeMap::new();
        let mut vcs_config = InMemory::new(&mut buffer);

        let relates = RelateTo::from("[#12345678]");
        let actual = set_relates_to(&mut vcs_config, &relates, Duration::from_hours(1));

        actual.unwrap();

        let sec59min = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|x| x.add(Duration::from_mins(59)))
            .map_err(|x| -> Box<dyn Error> { Box::from(x) })
            .map(|x| x.as_secs())
            .and_then(|x| i64::try_from(x).map_err(Box::from))
            .unwrap();

        let sec61min = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|x| x.add(Duration::from_mins(61)))
            .map_err(|x| -> Box<dyn Error> { Box::from(x) })
            .map(|x| x.as_secs())
            .and_then(|x| i64::try_from(x).map_err(Box::from))
            .unwrap();

        let actual_expire_time: i64 = buffer
            .get("mit.relate.expires")
            .and_then(|x| x.parse().ok())
            .expect("Failed to read expire");

        assert!(
            actual_expire_time < sec61min,
            "Expected less than {}, found {}",
            sec61min,
            actual_expire_time
        );
        assert!(
            actual_expire_time > sec59min,
            "Expected more than {}, found {}",
            sec59min,
            actual_expire_time,
        );
    }
}
