use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type TimestampUnix = u64;
pub type TimestampUTC = String;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TimelineEntry {
    pub timestamp: TimestampUnix,
    pub paststamp: TimestampUnix,
    pub pastdelta: TimestampUnix,

    pub views_u: u64,
    pub views_a: u64,

    pub change_a: i64,
    pub change_u: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Timeline {
    pub metrics_total: Metric,
    pub latest_entry_timestamp: TimestampUnix,
    pub metric_timeline: HashMap<TimestampUnix, TimelineEntry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Metric {
    pub all: u64,
    pub unique: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ViewCloneMetric {
    pub views: Metric,
    pub clones: Metric,
}

pub type ReferrerDomain = String;
pub type ContentPath = String;
pub type Map<K, V> = HashMap<K, V>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Report {
    pub total: ViewCloneMetric,
    pub referrals: Metric,
    pub content: Metric,
    pub referrers: u64,
    pub traffic_paths: u64,

    pub weekly: HashMap<TimestampUTC, ViewCloneMetric>,
    pub daily: HashMap<TimestampUTC, ViewCloneMetric>,

    pub timelines_referrals: Map<ReferrerDomain, Timeline>,
    pub timelines_content_traffic: Map<ContentPath, Timeline>,
}
