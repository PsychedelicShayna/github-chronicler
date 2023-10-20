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
    biweekly_views: ModelRepoViewsBiWeekly,
    biweekly_clones: ModelRepoClonesBiWeekly,
    daily_views: ModelRepoViewsDaily,
    daily_clones: ModelRepoClonesDaily,
    referrals: ModelReferrers,
    popular: ModelContentTrafficBiWeekly,
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
        biweekly_views,
        biweekly_clones,
        daily_views,
        daily_clones,
        referrals,
        popular,
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
    let unix_timestamp = get_unix_timestamp();

    for referral in &new_data.referrals {
        let referrer_domain = &referral.referrer;
        let new_referrals = referral.count;
        let unique_referrals = referral.uniques;

        let timeline = report
            .referrer_timelines
            .entry(referrer_domain.clone())
            .or_insert(Timeline {
                metrics_total: Metric { all: 0, unique: 0 },
                newest_metric: unix_timestamp,
                metric_timeline: Map::new(),
            });

        let old_entry = timeline.metric_timeline.get(&timeline.newest_metric);

        let time_now = unix_timestamp;
        let time_old = timeline.newest_metric;

        let time_delta = old_entry.map_or(0, |e| time_now - e.time_now);

        let timeline_entry = TimelineEntry {
            time_now,
            time_old,
            time_delta,
            new_unique: unique_referrals,
            new: new_referrals,
            delta: old_entry.map_or(0, |e| (new_referrals as i64 - e.new as i64) as u64),
            delta_unique: old_entry.map_or(0, |e| {
                (unique_referrals as i64 - e.new_unique as i64) as u64
            }),
        };

        timeline
            .metric_timeline
            .insert(unix_timestamp, timeline_entry);

        timeline.newest_metric = unix_timestamp;
        timeline.metrics_total.all = timeline.metrics_total.all.max(new_referrals);
        timeline.metrics_total.unique = timeline.metrics_total.unique.max(unique_referrals);
    }

    report.referrers = report.referrer_timelines.len() as u64;

    for content_path in &new_data.popular {
        let path = &content_path.path;
        let new_views = content_path.count;
        let unique_views = content_path.uniques;

        let timeline = report
            .content_timelines
            .entry(path.clone())
            .or_insert(Timeline {
                metrics_total: Metric { all: 0, unique: 0 },
                newest_metric: unix_timestamp,
                metric_timeline: Map::new(),
            });

        let old_entry = timeline.metric_timeline.get(&timeline.newest_metric);

        let time_now = unix_timestamp;
        let time_old = timeline.newest_metric;
        let time_delta = old_entry.map_or(0, |e| time_now - e.time_now);

        let timeline_entry = TimelineEntry {
            time_now,
            time_old,
            time_delta,
            new_unique: unique_views,
            new: new_views,
            delta: old_entry.map_or(0, |e| (new_views as i64 - e.new as i64) as u64),
            delta_unique: old_entry
                .map_or(0, |e| (unique_views as i64 - e.new_unique as i64) as u64),
        };

        timeline
            .metric_timeline
            .insert(unix_timestamp, timeline_entry);

        timeline.newest_metric = unix_timestamp;
        timeline.metrics_total.all = timeline.metrics_total.all.max(new_views);
        timeline.metrics_total.unique = timeline.metrics_total.unique.max(unique_views);
    }

    report.paths = report.content_timelines.len() as u64;

    for weekly_views in &new_data.biweekly_views.views {
        let timestamp = &weekly_views.timestamp;

        let entry = report
            .weekly
            .entry(timestamp.clone())
            .or_insert(ViewCloneMetric {
                views: Metric { all: 0, unique: 0 },
                clones: Metric { all: 0, unique: 0 },
            });

        entry.views.all = entry.views.all.max(weekly_views.count);
        entry.views.unique = entry.views.unique.max(weekly_views.uniques);
    }

    for weekly_clones in &new_data.biweekly_clones.clones {
        let timestamp = &weekly_clones.timestamp;

        let entry = report
            .weekly
            .entry(timestamp.clone())
            .or_insert(ViewCloneMetric {
                views: Metric { all: 0, unique: 0 },
                clones: Metric { all: 0, unique: 0 },
            });

        entry.clones.all = entry.clones.all.max(weekly_clones.count);
        entry.clones.unique = entry.clones.unique.max(weekly_clones.uniques);
    }

    for hourly_views in &new_data.daily_views.views {
        let timestamp = &hourly_views.timestamp;
        let entry = report
            .daily
            .entry(timestamp.clone())
            .or_insert(ViewCloneMetric {
                views: Metric { all: 0, unique: 0 },
                clones: Metric { all: 0, unique: 0 },
            });

        entry.views.all = entry.views.all.max(hourly_views.count);
        entry.views.unique = entry.views.unique.max(hourly_views.uniques);
    }

    for hourly_clones in &new_data.daily_clones.clones {
        let timestamp = &hourly_clones.timestamp;

        let entry = report
            .daily
            .entry(timestamp.clone())
            .or_insert(ViewCloneMetric {
                views: Metric { all: 0, unique: 0 },
                clones: Metric { all: 0, unique: 0 },
            });

        entry.clones.all = entry.clones.all.max(hourly_clones.count);
        entry.clones.unique = entry.clones.unique.max(hourly_clones.uniques);
    }

    let new_views_all = new_data.daily_views.count + new_data.biweekly_views.count;
    let new_clones_all = new_data.daily_clones.count + new_data.biweekly_clones.count;

    let new_views_unique = new_data.daily_views.uniques + new_data.biweekly_views.uniques;

    let new_clones_unique = new_data.daily_clones.uniques + new_data.biweekly_clones.uniques;

    report.total.views.all = report.total.views.all.max(new_views_all);
    report.total.views.unique = report.total.views.unique.max(new_views_unique);
    report.total.clones.all = report.total.clones.all.max(new_clones_all);
    report.total.clones.unique = report.total.clones.unique.max(new_clones_unique);

    report.referrals =
        report
            .referrer_timelines
            .iter()
            .fold(Metric { all: 0, unique: 0 }, |acc, kv| {
                let timeline = &kv.1;
                let metric = &timeline.metrics_total;

                Metric {
                    all: acc.all + metric.all,
                    unique: acc.unique + metric.unique,
                }
            });

    report.content =
        report
            .content_timelines
            .iter()
            .fold(Metric { all: 0, unique: 0 }, |acc, kv| {
                let timeline = &kv.1;
                let metric = &timeline.metrics_total;

                Metric {
                    all: acc.all + metric.all,
                    unique: acc.unique + metric.unique,
                }
            });

    Ok(report)
}

pub fn create_new_report(report_data: ReportData) -> Report {
    let mut report = Report {
        total: ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        },
        referrals: Metric { all: 0, unique: 0 },
        content: Metric { all: 0, unique: 0 },
        referrers: 0u64,
        paths: 0u64,
        weekly: Map::<TimestampUTC, ViewCloneMetric>::new(),
        daily: Map::<TimestampUTC, ViewCloneMetric>::new(),
        referrer_timelines: Map::<ReferrerDomain, Timeline>::new(),
        content_timelines: Map::<ContentPath, Timeline>::new(),
    };

    report.total.views.all = report_data.daily_views.count;
    report.total.views.unique = report_data.daily_views.uniques;
    report.total.views.all += report_data.biweekly_views.count;
    report.total.views.unique += report_data.biweekly_views.uniques;

    for referral in report_data.referrals {
        let referrer_domain = referral.referrer;
        let new_referrals = referral.count;
        let unique_referrals = referral.uniques;

        let unix_timestamp = get_unix_timestamp();

        let timeline = report
            .referrer_timelines
            .entry(referrer_domain)
            .or_insert(Timeline {
                metrics_total: Metric { all: 0, unique: 0 },
                newest_metric: unix_timestamp,
                metric_timeline: Map::<TimestampUnix, TimelineEntry>::new(),
            });

        let timeline_entry = TimelineEntry {
            time_now: unix_timestamp,
            time_old: unix_timestamp,
            time_delta: 0,
            new_unique: unique_referrals,
            new: new_referrals,
            delta: 0,
            delta_unique: 0,
        };

        timeline
            .metric_timeline
            .insert(unix_timestamp, timeline_entry);

        timeline.metrics_total.all += new_referrals;
        timeline.metrics_total.unique += unique_referrals;
    }

    report.referrers = report.referrer_timelines.len() as u64;

    for content_path in report_data.popular {
        let path = content_path.path;
        let new_views = content_path.count;
        let unique_views = content_path.uniques;

        let unix_timestamp = get_unix_timestamp();

        let timeline = report.content_timelines.entry(path).or_insert(Timeline {
            metrics_total: Metric { all: 0, unique: 0 },
            newest_metric: unix_timestamp,
            metric_timeline: Map::<TimestampUnix, TimelineEntry>::new(),
        });

        let timeline_entry = TimelineEntry {
            time_now: unix_timestamp,
            time_old: unix_timestamp,
            time_delta: 0,
            new_unique: unique_views,
            new: new_views,
            delta: 0,
            delta_unique: 0,
        };

        timeline
            .metric_timeline
            .insert(unix_timestamp, timeline_entry);

        timeline.metrics_total.all += new_views;
        timeline.metrics_total.unique += unique_views;
    }

    report.paths = report.content_timelines.len() as u64;

    for weekly_views in report_data.biweekly_views.views {
        let timestamp = weekly_views.timestamp;

        let entry = &mut report.weekly.entry(timestamp).or_insert(ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        });

        entry.views.all += weekly_views.count;
        entry.views.unique += weekly_views.uniques;
    }

    for weekly_clones in report_data.biweekly_clones.clones {
        let timestamp = weekly_clones.timestamp;

        let entry = &mut report.weekly.entry(timestamp).or_insert(ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        });

        entry.clones.all += weekly_clones.count;
        entry.clones.unique += weekly_clones.uniques;
    }

    for hourly_views in report_data.daily_views.views {
        let timestamp = hourly_views.timestamp;

        let entry = &mut report.daily.entry(timestamp).or_insert(ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        });

        entry.views.all += hourly_views.count;
        entry.views.unique += hourly_views.uniques;
    }

    for hourly_clones in report_data.daily_clones.clones {
        let timestamp = hourly_clones.timestamp;

        let entry = &mut report.daily.entry(timestamp).or_insert(ViewCloneMetric {
            views: Metric { all: 0, unique: 0 },
            clones: Metric { all: 0, unique: 0 },
        });

        entry.clones.all += hourly_clones.count;
        entry.clones.unique += hourly_clones.uniques;
    }

    report
}
