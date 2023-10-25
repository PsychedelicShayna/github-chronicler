use std::collections::HashMap;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use chrono::TimeZone;
use serde::{Deserialize, Serialize};

use crate::api::*;
use crate::api_models::*;
use chrono::{Datelike, Duration, Timelike, Utc};

use anyhow as ah;

pub type TimestampUnix = u64;
pub type TimestampUTC = String;

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

pub struct ReportData {
    pub biweekly_v: ModelRepoViewsBiWeekly,
    pub biweekly_c: ModelRepoClonesBiWeekly,
    pub daily_v: ModelRepoViewsDaily,
    pub daily_c: ModelRepoClonesDaily,
    pub referrals: ModelReferrerals,
    pub content_traffic: ModelContentTrafficBiWeekly,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct TimelineEntry {
    pub timestamp: TimestampUnix,
    pub paststamp: TimestampUnix,
    pub pastdelta: TimestampUnix,

    pub views_u: u64,
    pub views_a: u64,

    pub change_a: i64,
    pub change_u: i64,
}

impl TimelineEntry {
    pub fn update(
        &mut self,
        timestamp: TimestampUnix,
        views_a: u64,
        views_u: u64,
    ) -> ah::Result<()> {
        Ok(())
    }

    pub fn try_from_older(
        older: &Self,
        timestamp: TimestampUnix,
        views_a: u64,
        views_u: u64,
    ) -> Option<TimelineEntry> {
        let mut entry = TimelineEntry::default();

        let (min_a, max_a) = (&views_a.min(older.views_a), &views_a.max(older.views_a));
        let (min_u, max_u) = (&views_u.min(older.views_u), &views_u.max(older.views_u));

        entry.views_a = max_a - min_a;
        entry.views_u = max_u - min_u;

        if entry.views_a == 0 {
            return None;
        }

        entry.timestamp = timestamp;

        entry.paststamp = older.timestamp;
        entry.pastdelta = entry.timestamp - entry.paststamp;

        entry.change_a = views_a as i64 - older.views_a as i64;
        entry.change_u = views_u as i64 - older.views_u as i64;

        Some(entry)
    }

    pub fn new(timestamp: TimestampUnix, views_a: u64, views_u: u64) -> TimelineEntry {
        TimelineEntry {
            timestamp,
            paststamp: timestamp,
            pastdelta: 0,
            views_a,
            views_u,
            change_a: 0,
            change_u: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Timeline {
    pub metrics_total: Metric,
    pub latest_entry_timestamp: TimestampUnix,
    pub metric_timeline: HashMap<TimestampUnix, TimelineEntry>,
}

impl Default for Timeline {
    fn default() -> Self {
        Self {
            metrics_total: Metric::default(),
            latest_entry_timestamp: TimestampUnix::default(),
            metric_timeline: HashMap::<TimestampUnix, TimelineEntry>::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Metric {
    pub all: u64,
    pub unique: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ViewCloneMetric {
    pub views: Metric,
    pub clones: Metric,
}

pub type ReferrerDomain = String;
pub type ContentPath = String;
pub type Map<K, V> = HashMap<K, V>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct QuantifiableEvents {
    pub amount: u64,
    pub amount_unique: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct JsonReportModel {
    pub all_time_views: u64,
    pub all_time_views_unique: u64,

    pub all_time_clones: u64,
    pub all_time_clones_unique: u64,

    pub all_time_referrals: u64,
    pub all_time_referrals_unique: u64,

    pub all_time_content_visits: u64,
    pub all_time_content_visits_unique: u64,

    pub amount_of_referrers: u64,
    pub amount_of_content_visited: u64,

    pub weekly_views: HashMap<String, QuantifiableEvents>,
    pub weekly_clones: HashMap<String, QuantifiableEvents>,

    pub hourly_views: HashMap<String, QuantifiableEvents>,
    pub hourly_clones: HashMap<String, QuantifiableEvents>,

    pub weekly_referrals: HashMap<String, QuantifiableEvents>,
    pub weekly_content_visits: HashMap<String, QuantifiableEvents>,
}



#[derive(Debug, Deserialize, Serialize, Clone, Default)]
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

impl From<ReportData> for Report {
    fn from(value: ReportData) -> Self {
        Report::new(value)
    }
}

impl Report {
    pub fn new(data: ReportData) -> Self {
        let timestamp: u64 = get_unix_timestamp();

        let mut report = Report::default();

        report.total.views = Metric {
            all: data.daily_v.count + data.biweekly_v.count,
            unique: data.daily_v.uniques + data.biweekly_v.uniques,
        };

        report.total.clones = Metric {
            all: data.daily_c.count + data.biweekly_c.count,
            unique: data.daily_c.uniques + data.biweekly_c.uniques,
        };

        for referral in data.referrals {
            let mut views_a: u64 = referral.count;
            let mut views_u: u64 = referral.uniques;

            let mut change_a: i64 = 0;
            let mut change_u: i64 = 0;

            let timeline: &mut Timeline = report
                .timelines_referrals
                .entry(referral.referrer)
                .or_insert_with(|| Timeline::default());

            let entry = TimelineEntry::new(timestamp, views_a, views_u);
            timeline.metric_timeline.insert(timestamp, entry);
        }

        todo!()
    }

    pub fn update_clone(&self, new_data: &ReportData) -> ah::Result<Report> {
        let mut clone = self.clone();
        clone.update(new_data)?;
        Ok(clone)
    }

    pub fn update(&mut self, new_data: &ReportData) -> ah::Result<()> {
        let timestamp = get_unix_timestamp();

        for referral in &new_data.referrals {
            let timeline = self
                .timelines_referrals
                .entry(referral.referrer.clone())
                .or_insert_with(|| Timeline::default());

            let previous_entry = timeline
                .metric_timeline
                .get(&timeline.latest_entry_timestamp);

            let mut updated_entry: Option<TimelineEntry> = None;

            if let Some(previous) = previous_entry {
                updated_entry = TimelineEntry::try_from_older(
                    previous,
                    timestamp,
                    referral.count,
                    referral.uniques,
                );
            }

            let updated_entry = match updated_entry {
                Some(entry) => entry,
                None => continue,
            };

            timeline.metric_timeline.insert(timestamp, updated_entry);
            timeline.latest_entry_timestamp = timestamp;

            timeline.metrics_total.all = timeline.metrics_total.all.max(referral.count);
            timeline.metrics_total.unique = timeline.metrics_total.unique.max(referral.uniques);
        }

        for content_traffic in &new_data.content_traffic {
            let traffic_path = &content_traffic.path;

            let timeline = self
                .timelines_content_traffic
                .entry(traffic_path.clone())
                .or_insert(Timeline {
                    metrics_total: Metric { all: 0, unique: 0 },
                    latest_entry_timestamp: timestamp,
                    metric_timeline: Map::new(),
                });

            let previous_entry = timeline
                .metric_timeline
                .get(&timeline.latest_entry_timestamp);

            let mut discard_entry: bool = false;

            let mut views_a: u64 = content_traffic.count;
            let mut views_u: u64 = content_traffic.uniques;

            let mut change_a: i64 = 0;
            let mut change_u: i64 = 0;

            if let Some(previous) = previous_entry {
                let old_views_a = previous.views_a;
                let old_views_u = previous.views_u;

                let (min_a, max_a) = (&views_a.min(old_views_a), &views_a.max(old_views_a));
                let (min_u, max_u) = (&views_u.min(old_views_u), &views_u.max(old_views_u));

                views_a = max_a - min_a;
                views_u = max_u - min_u;

                if views_a == 0 {
                    discard_entry = true;
                }

                change_a = views_a as i64 - old_views_a as i64;
                change_u = views_u as i64 - old_views_u as i64;
            }

            if discard_entry {
                continue;
            }

            let paststamp: u64 = previous_entry.map_or(0, |e| e.timestamp);
            let pastdelta: u64 = timestamp - paststamp;

            let timeline_entry = TimelineEntry {
                timestamp,
                paststamp,
                pastdelta,
                views_u,
                views_a,
                change_a,
                change_u,
            };

            timeline.metric_timeline.insert(timestamp, timeline_entry);
            timeline.latest_entry_timestamp = timestamp;

            timeline.metrics_total.all = timeline.metrics_total.all.max(views_a);
            timeline.metrics_total.unique = timeline.metrics_total.unique.max(views_u);
        }

        // Update weekly view metrics.
        for week in &new_data.biweekly_v.views {
            let timestamp = &week.timestamp;

            let entry = self
                .weekly
                .entry(timestamp.clone())
                .or_insert(ViewCloneMetric {
                    views: Metric { all: 0, unique: 0 },
                    clones: Metric { all: 0, unique: 0 },
                });

            entry.views.all = entry.views.all.max(week.count);
            entry.views.unique = entry.views.unique.max(week.uniques);
        }

        // Update weekly clone metrics.
        for week in &new_data.biweekly_c.clones {
            let entry = self
                .weekly
                .entry(week.timestamp.clone())
                .or_insert(ViewCloneMetric {
                    views: Metric { all: 0, unique: 0 },
                    clones: Metric { all: 0, unique: 0 },
                });

            entry.clones.all = entry.clones.all.max(week.count);
            entry.clones.unique = entry.clones.unique.max(week.uniques);
        }

        // Update hourly view metrics.
        for hour in &new_data.daily_v.views {
            let entry = self
                .daily
                .entry(hour.timestamp.clone())
                .or_insert(ViewCloneMetric {
                    views: Metric { all: 0, unique: 0 },
                    clones: Metric { all: 0, unique: 0 },
                });

            entry.views.all = entry.views.all.max(hour.count);
            entry.views.unique = entry.views.unique.max(hour.uniques);
        }

        // Update hourly clone metrics.
        for hour in &new_data.daily_c.clones {
            let entry = self
                .daily
                .entry(hour.timestamp.clone())
                .or_insert(ViewCloneMetric {
                    views: Metric { all: 0, unique: 0 },
                    clones: Metric { all: 0, unique: 0 },
                });

            entry.clones.all = entry.clones.all.max(hour.count);
            entry.clones.unique = entry.clones.unique.max(hour.uniques);
        }

        // Deal with updating the view count.
        let views_a = new_data.daily_v.count + new_data.biweekly_v.count;
        let views_u = new_data.daily_v.uniques + new_data.biweekly_v.uniques;

        self.total.views.all = self.total.views.all.max(views_a);
        self.total.views.unique = self.total.views.unique.max(views_u);

        // Deal with updating the clone count.
        let clones_a = new_data.daily_c.count + new_data.biweekly_c.count;
        let clones_u = new_data.daily_c.uniques + new_data.biweekly_c.uniques;

        self.total.clones.all = self.total.clones.all.max(clones_a);
        self.total.clones.unique = self.total.clones.unique.max(clones_u);

        // Count total referrals from different referrers.
        self.referrals = self.timelines_referrals.iter().fold(
            Metric { all: 0, unique: 0 },
            |acc, (_, timeline)| Metric {
                all: acc.all + timeline.metrics_total.all,
                unique: acc.unique + timeline.metrics_total.unique,
            },
        );

        // Count total visits to different content paths.
        self.content = self.timelines_content_traffic.iter().fold(
            Metric { all: 0, unique: 0 },
            |acc, (_, timeline)| Metric {
                all: acc.all + timeline.metrics_total.all,
                unique: acc.unique + timeline.metrics_total.unique,
            },
        );

        // Count the number of unique traffic paths and referrering websites.
        self.traffic_paths = self.timelines_content_traffic.len() as u64;
        self.referrers = self.timelines_referrals.len() as u64;

        Ok(())
    }
}
