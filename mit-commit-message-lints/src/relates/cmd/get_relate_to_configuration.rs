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
pub fn get_relate_to_configuration(config: &mut dyn Vcs) -> Result<Option<RelateTo>> {
    let config_value = config.get_i64(CONFIG_KEY_EXPIRES)?;

    match config_value {
        Some(config_value) => {
            let now = now()?;

            if now < Duration::from_secs(config_value.try_into().into_diagnostic()?) {
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

    use crate::{
        external::InMemory,
        relates::{get_relate_to_configuration, RelateTo},
    };

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
        );
    }

    #[test]
    fn we_get_relate_to_config_back_if_there_is_any() {
        let mut buffer = BTreeMap::new();
        buffer.insert(
            "mit.relate.expires".into(),
            format!("{}", epoch_with_offset(add_10_seconds)),
        );
        buffer.insert("mit.relate.to".into(), "[#12345678]".into());
        let mut vcs = InMemory::new(&mut buffer);

        let actual = get_relate_to_configuration(&mut vcs).expect("Failed to read VCS config");
        let expected = Some(RelateTo::new("[#12345678]"));

        assert_eq!(
            expected, actual,
            "Expected the relate config to be {:?}, instead got {:?}",
            expected, actual
        );
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
