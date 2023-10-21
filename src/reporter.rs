use crate::{api::*, api_models::*, report_models::*};

use std::{
    io::{Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow as ah;

pub fn get_unix_timestamp() -> u64 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime before Unix epoch!");

    duration_since_epoch.as_secs()
}

pub struct ReportData {
    biweekly_v: ModelRepoViewsBiWeekly,
    biweekly_c: ModelRepoClonesBiWeekly,
    daily_v: ModelRepoViewsDaily,
    daily_c: ModelRepoClonesDaily,
    referrals: ModelReferrerals,
    content_traffic: ModelContentTrafficBiWeekly,
}

pub fn request_report_data(
    token: &AuthToken,
    author: &String,
    repo: &String,
) -> ah::Result<ReportData> {
    let biweekly_views = request_views_weekly(token, author, repo)?;
    println!("Biweekly Views: {:?}", biweekly_views);

    let biweekly_clones = request_clones_weekly(token, author, repo)?;
    println!("biweekly_clones: {:?}", biweekly_clones);

    let daily_views = request_views_daily(token, author, repo)?;
    println!("daily_views: {:?}", daily_views);

    let daily_clones = request_clones_daily(token, author, repo)?;
    println!("daily_clones: {:?}", daily_clones);

    let referrals = request_referrers_weekly(token, author, repo)?;
    println!("referrals: {:?}", referrals);

    let popular = request_popular_paths_weekly(token, author, repo)?;
    println!("popular: {:?}", popular);

    Ok(ReportData {
        biweekly_v: biweekly_views,
        biweekly_c: biweekly_clones,
        daily_v: daily_views,
        daily_c: daily_clones,
        referrals,
        content_traffic: popular,
    })
}

pub fn save_report_file(report: Report, file_path: &String) -> ah::Result<()> {
    let report_json = sj::to_string(&report)?;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(report_json.as_bytes())?;

    Ok(())
}

pub fn load_report_file(file_path: &String) -> ah::Result<Report> {
    let mut file = std::fs::OpenOptions::new().read(true).open(file_path)?;
    let mut buffer: String = String::new();

    file.read_to_string(&mut buffer)?;
    let report: Report = sj::from_str(buffer.as_str())?;

    Ok(report)
}

pub fn update_existing_report<'a>(mut report: Report, new_data: &ReportData) -> ah::Result<Report> {
    let timestamp = get_unix_timestamp();

    for referral in &new_data.referrals {
        let referrer = &referral.referrer;

        let timeline = report
            .timelines_referrals
            .entry(referrer.clone())
            .or_insert(Timeline {
                metrics_total: Metric { all: 0, unique: 0 },
                latest_entry_timestamp: timestamp,
                metric_timeline: Map::new(),
            });

        let previous_entry = timeline
            .metric_timeline
            .get(&timeline.latest_entry_timestamp);

        let mut discard_entry: bool = false;

        let mut views_a: u64 = referral.count;
        let mut views_u: u64 = referral.uniques;

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

    for content_traffic in &new_data.content_traffic {
        let traffic_path = &content_traffic.path;

        let timeline = report
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

        let entry = report
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
        let entry = report
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
        let entry = report
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
        let entry = report
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

    report.total.views.all = report.total.views.all.max(views_a);
    report.total.views.unique = report.total.views.unique.max(views_u);

    // Deal with updating the clone count.
    let clones_a = new_data.daily_c.count + new_data.biweekly_c.count;
    let clones_u = new_data.daily_c.uniques + new_data.biweekly_c.uniques;

    report.total.clones.all = report.total.clones.all.max(clones_a);
    report.total.clones.unique = report.total.clones.unique.max(clones_u);

    // Count total referrals from different referrers.
    report.referrals = report.timelines_referrals.iter().fold(
        Metric { all: 0, unique: 0 },
        |acc, (_, timeline)| Metric {
            all: acc.all + timeline.metrics_total.all,
            unique: acc.unique + timeline.metrics_total.unique,
        },
    );

    // Count total visits to different content paths.
    report.content = report.timelines_content_traffic.iter().fold(
        Metric { all: 0, unique: 0 },
        |acc, (_, timeline)| Metric {
            all: acc.all + timeline.metrics_total.all,
            unique: acc.unique + timeline.metrics_total.unique,
        },
    );

    // Count the number of unique traffic paths and referrering websites.
    report.traffic_paths = report.timelines_content_traffic.len() as u64;
    report.referrers = report.timelines_referrals.len() as u64;

    Ok(report)
}

pub fn generate_new_report(report_data: ReportData) -> Report {
    let mut report = Report {
        total: ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        },
        referrals: Metric { all: 0, unique: 0 },
        content: Metric { all: 0, unique: 0 },
        referrers: 0u64,
        traffic_paths: 0u64,
        weekly: Map::<TimestampUTC, ViewCloneMetric>::new(),
        daily: Map::<TimestampUTC, ViewCloneMetric>::new(),
        timelines_referrals: Map::<ReferrerDomain, Timeline>::new(),
        timelines_content_traffic: Map::<ContentPath, Timeline>::new(),
    };

    report.total.views.all = report_data.daily_v.count;
    report.total.views.unique = report_data.daily_v.uniques;
    report.total.views.all += report_data.biweekly_v.count;
    report.total.views.unique += report_data.biweekly_v.uniques;

    for referral in report_data.referrals {
        let referrer_domain = referral.referrer;
        let new_referrals = referral.count;
        let unique_referrals = referral.uniques;

        let unix_timestamp = get_unix_timestamp();

        let timeline = report
            .timelines_referrals
            .entry(referrer_domain)
            .or_insert(Timeline {
                metrics_total: Metric { all: 0, unique: 0 },
                latest_entry_timestamp: unix_timestamp,
                metric_timeline: Map::<TimestampUnix, TimelineEntry>::new(),
            });

        let timeline_entry = TimelineEntry {
            timestamp: unix_timestamp,
            paststamp: unix_timestamp,
            pastdelta: 0,
            views_u: unique_referrals,
            views_a: new_referrals,
            change_a: 0,
            change_u: 0,
        };

        timeline
            .metric_timeline
            .insert(unix_timestamp, timeline_entry);

        timeline.metrics_total.all += new_referrals;
        timeline.metrics_total.unique += unique_referrals;
    }

    report.referrers = report.timelines_referrals.len() as u64;

    for content_path in report_data.content_traffic {
        let path = content_path.path;
        let new_views = content_path.count;
        let unique_views = content_path.uniques;

        let unix_timestamp = get_unix_timestamp();

        let timeline = report
            .timelines_content_traffic
            .entry(path)
            .or_insert(Timeline {
                metrics_total: Metric { all: 0, unique: 0 },
                latest_entry_timestamp: unix_timestamp,
                metric_timeline: Map::<TimestampUnix, TimelineEntry>::new(),
            });

        let timeline_entry = TimelineEntry {
            timestamp: unix_timestamp,
            paststamp: unix_timestamp,
            pastdelta: 0,
            views_u: unique_views,
            views_a: new_views,
            change_a: 0,
            change_u: 0,
        };

        timeline
            .metric_timeline
            .insert(unix_timestamp, timeline_entry);

        timeline.metrics_total.all += new_views;
        timeline.metrics_total.unique += unique_views;
    }

    report.traffic_paths = report.timelines_content_traffic.len() as u64;

    for weekly_views in report_data.biweekly_v.views {
        let timestamp = weekly_views.timestamp;

        let entry = &mut report.weekly.entry(timestamp).or_insert(ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        });

        entry.views.all += weekly_views.count;
        entry.views.unique += weekly_views.uniques;
    }

    for weekly_clones in report_data.biweekly_c.clones {
        let timestamp = weekly_clones.timestamp;

        let entry = &mut report.weekly.entry(timestamp).or_insert(ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        });

        entry.clones.all += weekly_clones.count;
        entry.clones.unique += weekly_clones.uniques;
    }

    for hourly_views in report_data.daily_v.views {
        let timestamp = hourly_views.timestamp;

        let entry = &mut report.daily.entry(timestamp).or_insert(ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        });

        entry.views.all += hourly_views.count;
        entry.views.unique += hourly_views.uniques;
    }

    for hourly_clones in report_data.daily_c.clones {
        let timestamp = hourly_clones.timestamp;

        let entry = &mut report.daily.entry(timestamp).or_insert(ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        });

        entry.clones.all += hourly_clones.count;
        entry.clones.unique += hourly_clones.uniques;
    }

    for (_, timeline) in &report.timelines_referrals {
        report.referrals.all += timeline.metrics_total.all;
        report.referrals.unique += timeline.metrics_total.all;
    }

    for (_, timeline) in &report.timelines_content_traffic {
        report.content.all += timeline.metrics_total.all;
        report.content.unique += timeline.metrics_total.unique;
    }

    report
}
