use std::{
    num,
    ops::Add,
    option::Option,
    result::Result,
    time,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::entities::RelateTo;
use crate::external;
use crate::external::Vcs;
use std::{convert::TryInto, time::SystemTimeError};
use thiserror::Error;

const CONFIG_KEY_EXPIRES: &str = "mit.relate.expires";

/// Get the relate-to that are currently defined for this vcs config source
///
/// # Errors
///
/// Will fail if reading or writing from the VCS config fails, or it contains
/// data in an incorrect format
pub fn get_relate_to_configuration(config: &mut dyn Vcs) -> Result<Option<RelateTo>, Error> {
    let config_value = config.get_i64(CONFIG_KEY_EXPIRES)?;

    match config_value {
        Some(config_value) => {
            let now = now()?;

            if now < Duration::from_secs(config_value.try_into()?) {
                let relate_to_config = get_vcs_relate_to(config)?.map(RelateTo::new);

                Ok(relate_to_config)
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests_able_to_load_config_from_git {
    use std::{
        collections::BTreeMap,
        convert::TryFrom,
        ops::{Add, Sub},
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use pretty_assertions::assert_eq;

    use crate::external::InMemory;
    use crate::relates::entities::RelateTo;
    use crate::relates::vcs::get_relate_to_configuration;

    #[test]
    fn there_is_no_relate_config_if_it_has_expired() {
        let now_minus_10 = epoch_with_offset(subtract_10_seconds);
        let mut strings: BTreeMap<String, String> = BTreeMap::new();
        strings.insert("mit.relate.expires".into(), format!("{}", now_minus_10));
        let mut vcs = InMemory::new(&mut strings);

        let actual = get_relate_to_configuration(&mut vcs).expect("Failed to read VCS config");
        let expected = None;
        assert_eq!(
            expected, actual,
            "Expected the relate config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    #[test]
    fn we_get_relate_to_config_back_if_there_is_any() {
        let mut strs = BTreeMap::new();
        strs.insert(
            "mit.relate.expires".into(),
            format!("{}", epoch_with_offset(add_10_seconds)),
        );
        strs.insert("mit.relate.to".into(), "[#12345678]".into());
        let mut vcs = InMemory::new(&mut strs);

        let actual = get_relate_to_configuration(&mut vcs).expect("Failed to read VCS config");
        let expected = Some(RelateTo::new("[#12345678]"));

        assert_eq!(
            expected, actual,
            "Expected the relate config to be {:?}, instead got {:?}",
            expected, actual
        )
    }

    fn add_10_seconds(x: Duration) -> Duration {
        x.add(Duration::from_secs(10))
    }

    fn subtract_10_seconds(x: Duration) -> Duration {
        x.sub(Duration::from_secs(10))
    }

    fn into_seconds(x: Duration) -> u64 {
        x.as_secs()
    }

    fn epoch_with_offset(x: fn(Duration) -> Duration) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(x)
            .map(into_seconds)
            .map(i64::try_from)
            .expect("Failed to get Unix Epoch")
            .expect("Convert epoch to int")
    }
}

fn now() -> Result<Duration, SystemTimeError> {
    SystemTime::now().duration_since(UNIX_EPOCH)
}

#[allow(clippy::maybe_infinite_iter)]
fn get_vcs_relate_to(config: &dyn Vcs) -> Result<Option<&str>, Error> {
    config.get_str("mit.relate.to").map_err(Error::from)
}

/// # Errors
///
/// This errors if writing to the git mit file fails for some reason. Those
/// reasons will be specific to VCS implementation
pub fn set_relates_to(
    config: &mut dyn Vcs,
    relates: &RelateTo,
    expires_in: Duration,
) -> Result<(), Error> {
    set_vcs_relates_to(config, relates)?;
    set_vcs_expires_time(config, expires_in)?;

    Ok(())
}

#[cfg(test)]
mod tests_can_set_relates_to_details {
    use std::{
        collections::BTreeMap,
        convert::TryFrom,
        error::Error,
        ops::Add,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use crate::external::InMemory;
    use crate::relates::entities::RelateTo;
    use crate::relates::vcs::set_relates_to;

    #[test]
    fn the_first_initial_becomes_the_relates() {
        let mut strs = BTreeMap::new();

        let mut vcs_config = InMemory::new(&mut strs);

        let relates_to = RelateTo::new("[#12345678]");
        let actual = set_relates_to(&mut vcs_config, &relates_to, Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());
        assert_eq!(Some(&"[#12345678]".to_string()), strs.get("mit.relate.to"));
    }

    #[test]
    fn sets_the_expiry_time() {
        let mut strs = BTreeMap::new();
        let mut vcs_config = InMemory::new(&mut strs);

        let relates = RelateTo::new("[#12345678]");
        let actual = set_relates_to(&mut vcs_config, &relates, Duration::from_secs(60 * 60));

        assert_eq!(true, actual.is_ok());

        let sec59min = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|x| x.add(Duration::from_secs(60 * 59)))
            .map_err(|x| -> Box<dyn Error> { Box::from(x) })
            .map(|x| x.as_secs())
            .and_then(|x| i64::try_from(x).map_err(Box::from))
            .unwrap();

        let sec61min = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|x| x.add(Duration::from_secs(60 * 61)))
            .map_err(|x| -> Box<dyn Error> { Box::from(x) })
            .map(|x| x.as_secs())
            .and_then(|x| i64::try_from(x).map_err(Box::from))
            .unwrap();

        let actual_expire_time: i64 = strs
            .get("mit.relate.expires")
            .and_then(|x| x.parse().ok())
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
    }
}

fn set_vcs_relates_to(config: &mut dyn Vcs, relates: &RelateTo) -> Result<(), Error> {
    config.set_str("mit.relate.to", &relates.to())?;
    Ok(())
}

fn set_vcs_expires_time(config: &mut dyn Vcs, expires_in: Duration) -> Result<(), Error> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
    let expiry_time = now.add(expires_in).as_secs().try_into()?;
    config
        .set_i64(CONFIG_KEY_EXPIRES, expiry_time)
        .map_err(Error::from)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to talk to the gitr config: {0}")]
    GitIo(#[from] external::Error),
    #[error("failed converted epoch int between types: {0}")]
    EpochConvert(#[from] num::TryFromIntError),
    #[error("failed to get system time: {0}")]
    SystemTime(#[from] time::SystemTimeError),
}
