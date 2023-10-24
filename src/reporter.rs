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
todo!()
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
            .or_insert_with(|| Timeline::default() );

        let timeline_entry = TimelineEntry {
            timestamp: unix_timestamp,
            paststamp: unix_timestamp,
            pastdelta: 0,
            views_a: new_referrals,
            views_u: unique_referrals,
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
