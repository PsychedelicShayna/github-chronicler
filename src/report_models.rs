use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type TimestampUnix = u64;
pub type TimestampUTC = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimelineEntry {
    pub time_now: TimestampUnix,
    pub time_old: TimestampUnix,
    pub time_delta: TimestampUnix,

    pub new_unique: u64,
    pub new: u64,

    pub delta: u64,
    pub delta_unique: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Timeline {
    pub metrics_total: Metric,
    pub newest_metric: TimestampUnix,
    pub metric_timeline: HashMap<TimestampUnix, TimelineEntry>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Metric {
    pub all: u64,
    pub unique: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ViewCloneMetric {
    pub views: Metric,
    pub clones: Metric,
}

pub type ReferrerDomain = String;
pub type ContentPath = String;
pub type Map<K, V> = HashMap<K, V>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    pub total: ViewCloneMetric,
    pub referrals: Metric,
    pub content: Metric,
    pub referrers: u64,
    pub paths: u64,

    pub weekly: HashMap<TimestampUTC, ViewCloneMetric>,
    pub daily: HashMap<TimestampUTC, ViewCloneMetric>,

    pub referrer_timelines: Map<ReferrerDomain, Timeline>,
    pub content_timelines: Map<ContentPath, Timeline>,
}
