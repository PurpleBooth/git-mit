use std::{
    convert::TryFrom,
    time::{Duration, SystemTime},
};

use git2::Config;

pub fn get_author_configuration(config: &Config) -> std::option::Option<()> {
    if let Ok(true) = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|_err| false)
        .and_then(|time_since_epoch| -> Result<(Duration, Duration), bool> {
            config
                .get_i64("pb.author.expires")
                .map_err(|_err| false)
                .and_then(|x| u64::try_from(x).map_err(|_x| false))
                .map(Duration::from_secs)
                .map(|expires_after_time| (time_since_epoch, expires_after_time))
                .map_err(|_err| -> bool { false })
        })
        .map(|(time_since_epoch, expires_after_time)| time_since_epoch.lt(&expires_after_time))
    {
        return Some(());
    }

    None
}
