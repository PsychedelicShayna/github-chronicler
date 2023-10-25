use anyhow as ah;
use chrono::{prelude::*, Duration};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_utc_datestamp() -> ah::Result<String> {
    let utc_now = Utc::now();

    let midnight_aligned = Utc
        .with_ymd_and_hms(utc_now.year(), utc_now.month(), utc_now.day(), 0, 0, 0)
        .single();

    midnight_aligned
        .ok_or_else(|| {
            ah::anyhow!(format!(
                "Failed to parse timestamp {}, option was None",
                utc_now
            ))
        })
        .map(|time| time.format("%Y-%m-%dT%H:%M:%SZ").to_string())
}

pub fn next_utc_day() -> ah::Result<Duration> {
    let now = Utc::now();

    match Utc.with_ymd_and_hms(now.year(), now.month(), now.day() + 1, 0, 0, 0) {
        chrono::LocalResult::None => ah::bail!(""),
        chrono::LocalResult::Single(r) => Ok(r),
        chrono::LocalResult::Ambiguous(r1, _) => Ok(r1),
    }
    .map(|t| t.signed_duration_since(now))
}

pub fn get_unix_timestamp() -> u64 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before Unix epoch!");

    duration_since_epoch.as_secs()
}

pub fn subtract_two_weeks(timestamp: &String) -> ah::Result<String> {
    DateTime::parse_from_rfc3339(timestamp).map(|time| {
        time.with_day(time.with_timezone(&Utc).day() - 14)
            .map(|past| past.format("%Y-%m-%dT%H:%M:%SZ").to_string())
            .ok_or_else(|| {
                ah::anyhow!(format!(
                    "Failed to parse timestamp {}, option was None",
                    timestamp
                ))
            })
    })?
}
