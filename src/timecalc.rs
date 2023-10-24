use anyhow as ah;
use chrono::{
    format::{parse, Parsed},
    prelude::*,
};

pub fn get_day_utc() -> ah::Result<String> {
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
