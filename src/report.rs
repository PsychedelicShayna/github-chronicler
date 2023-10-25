use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::api::*;
use crate::api_models::*;
use crate::timecalc::*;

use anyhow as ah;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct QuantifiableEvents {
    pub amount: u64,
    pub amount_unique: u64,
}

type Referrer = String;
type ContentPath = String;
type DatestampUtc = String;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct RepositoryTrafficChronicle {
    pub total_views: u64,
    pub total_views_unique: u64,

    pub total_clones: u64,
    pub total_clones_unique: u64,

    pub total_content_visits: u64,
    pub total_content_visits_unique: u64,

    pub total_referrals: u64,
    pub total_referrals_unique: u64,

    pub all_time_referrals: HashMap<Referrer, QuantifiableEvents>,
    pub all_time_content_paths: HashMap<ContentPath, QuantifiableEvents>,

    pub hourly_views: HashMap<DatestampUtc, QuantifiableEvents>,
    pub hourly_clones: HashMap<DatestampUtc, QuantifiableEvents>,

    pub weekly_views: HashMap<DatestampUtc, QuantifiableEvents>,
    pub weekly_clones: HashMap<DatestampUtc, QuantifiableEvents>,

    pub weekly_referrals: HashMap<Referrer, HashMap<DatestampUtc, QuantifiableEvents>>,
    pub weekly_content_visits: HashMap<ContentPath, HashMap<DatestampUtc, QuantifiableEvents>>,
}

impl RepositoryTrafficChronicle {
    pub fn save_json_file(&self, file_path: &str) -> ah::Result<()> {
        let file = std::fs::File::create(file_path)?;
        Ok(serde_json::to_writer(file, self)?)
    }

    pub fn load_json_file(file_path: &str) -> ah::Result<Self> {
        let file = std::fs::File::open(file_path)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn request_new(
        token: &AuthToken,
        author: &String,
        repository: &String,
    ) -> ah::Result<Self> {
        let api_data_report = ApiDataReport::request(token, author, repository)?;
        RepositoryTrafficChronicle::new(&api_data_report)
    }

    pub fn request_update(
        &mut self,
        token: &AuthToken,
        author: &String,
        repository: &String,
    ) -> ah::Result<()> {
        let api_data_report = ApiDataReport::request(token, author, repository)?;
        self.update(&api_data_report)
    }

    pub fn update(&mut self, api_data: &ApiDataReport) -> ah::Result<()> {
        let timestamp = get_utc_datestamp()?;
        let fourteen_days_ago = subtract_two_weeks(&timestamp)?;

        // Update weekly views.
        for new_week in &api_data.biweekly_views_model.views {
            let new_week_views = new_week.count;
            let new_week_views_unique = new_week.uniques;
            let new_week_timestamp = &new_week.timestamp;

            let week_entry = self.weekly_views.get_mut(new_week_timestamp);

            if let Some(week_entry) = week_entry {
                week_entry.amount = week_entry.amount.max(new_week.count);
                week_entry.amount_unique = week_entry.amount_unique.max(new_week.uniques);
            } else {
                let new_week_data = QuantifiableEvents {
                    amount: new_week_views,
                    amount_unique: new_week_views_unique,
                };

                self.weekly_views
                    .insert(new_week_timestamp.clone(), new_week_data);
            }
        }

        // Update weekly clones.
        for new_week in &api_data.biweekly_clones_model.clones {
            let new_week_clones = new_week.count;
            let new_week_clones_unique = new_week.uniques;
            let new_week_timestamp = &new_week.timestamp;

            let week_entry = self.weekly_clones.get_mut(new_week_timestamp);

            if let Some(week_entry) = week_entry {
                week_entry.amount = week_entry.amount.max(new_week.count);
                week_entry.amount_unique = week_entry.amount_unique.max(new_week.uniques);
            } else {
                let new_week_data = QuantifiableEvents {
                    amount: new_week_clones,
                    amount_unique: new_week_clones_unique,
                };

                self.weekly_clones
                    .insert(new_week_timestamp.clone(), new_week_data);
            }
        }

        // Update hourly views.
        for new_hour in &api_data.daily_views_model.views {
            let new_hour_views = new_hour.count;
            let new_hour_views_unique = new_hour.uniques;
            let new_hour_timestamp = &new_hour.timestamp;

            let hour_entry = self.hourly_views.get_mut(new_hour_timestamp);

            if let Some(hour_entry) = hour_entry {
                if new_hour_views > hour_entry.amount {
                    self.total_views += new_hour_views;
                }

                if new_hour_views_unique > hour_entry.amount_unique {
                    self.total_views_unique += new_hour_views_unique;
                }

                hour_entry.amount = hour_entry.amount.max(new_hour_views);
                hour_entry.amount_unique = hour_entry.amount_unique.max(new_hour_views_unique);
            } else {
                let new_hour_data = QuantifiableEvents {
                    amount: new_hour_views,
                    amount_unique: new_hour_views_unique,
                };

                self.hourly_views
                    .insert(new_hour_timestamp.clone(), new_hour_data);
                self.total_views += new_hour_views;
                self.total_views_unique += new_hour_views_unique;
            }
        }

        // Update hourly clones.
        for new_hour in &api_data.daily_clones_model.clones {
            let new_hour_clones = new_hour.count;
            let new_hour_clones_unique = new_hour.uniques;
            let new_hour_timestamp = &new_hour.timestamp;

            let hour_entry = self.hourly_clones.get_mut(new_hour_timestamp);

            if let Some(hour_entry) = hour_entry {
                if new_hour_clones > hour_entry.amount {
                    self.total_clones += new_hour_clones;
                }

                if new_hour_clones_unique > hour_entry.amount_unique {
                    self.total_clones_unique += new_hour_clones_unique;
                }

                hour_entry.amount = hour_entry.amount.max(new_hour_clones);
                hour_entry.amount_unique = hour_entry.amount_unique.max(new_hour_clones_unique);
            } else {
                let new_hour_data = QuantifiableEvents {
                    amount: new_hour_clones,
                    amount_unique: new_hour_clones_unique,
                };

                self.hourly_clones
                    .insert(new_hour_timestamp.clone(), new_hour_data);
                self.total_clones += new_hour_clones;
                self.total_clones_unique += new_hour_clones_unique;
            }
        }

        for new_referral in &api_data.biweekly_referrals_model {
            let referrals = self
                .weekly_referrals
                .entry(new_referral.referrer.clone())
                .or_insert(HashMap::new());

            let referral = referrals.get_mut(&fourteen_days_ago);

            // Store the referrals for the last week in this referrer's timeline hashmap,
            // with a Datestamp as a key, being the date of the day 14 days ago.
            if let Some(referral) = referral {
                referral.amount += new_referral.count;
                referral.amount_unique += new_referral.uniques;
            } else {
                referrals.insert(
                    fourteen_days_ago.clone(),
                    QuantifiableEvents {
                        amount: new_referral.count,
                        amount_unique: new_referral.uniques,
                    },
                );
            }

            // Count the referrer in the HashMap of referrers, together with the total
            // amount of referrals gained from this referrer, or the existing count.
            if let Some(referral) = self.all_time_referrals.get_mut(&new_referral.referrer) {
                referral.amount += new_referral.count;
                referral.amount_unique += new_referral.uniques;
            } else {
                self.all_time_referrals.insert(
                    new_referral.referrer.clone(),
                    QuantifiableEvents {
                        amount: new_referral.count,
                        amount_unique: new_referral.uniques,
                    },
                );
            }
        }

        for new_content_visit in &api_data.biweekly_content_visits_model {
            let content_visits = self
                .weekly_content_visits
                .entry(new_content_visit.path.clone())
                .or_insert(HashMap::new());

            let content_visit = content_visits.get_mut(&fourteen_days_ago);

            // Store the content_visits for the last week in this referrer's timeline hashmap,
            // with a Datestamp as a key, being the date of the day 14 days ago.
            if let Some(content_visit) = content_visit {
                content_visit.amount += new_content_visit.count;
                content_visit.amount_unique += new_content_visit.uniques;
            } else {
                content_visits.insert(
                    fourteen_days_ago.clone(),
                    QuantifiableEvents {
                        amount: new_content_visit.count,
                        amount_unique: new_content_visit.uniques,
                    },
                );
            }

            // Count the referrer in the HashMap of referrers, together with the total
            // amount of content_visits gained from this referrer, or the existing count.
            if let Some(content_visit) =
                self.all_time_content_paths.get_mut(&new_content_visit.path)
            {
                content_visit.amount += new_content_visit.count;
                content_visit.amount_unique += new_content_visit.uniques;
            } else {
                self.all_time_content_paths.insert(
                    new_content_visit.path.clone(),
                    QuantifiableEvents {
                        amount: new_content_visit.count,
                        amount_unique: new_content_visit.uniques,
                    },
                );
            }
        }

        Ok(())
    }

    pub fn new(api_data: &ApiDataReport) -> ah::Result<Self> {
        let fourteen_days_ago = subtract_two_weeks(&get_utc_datestamp()?)?;

        let all_time_views = api_data.daily_views_model.count;
        let all_time_views_unique = api_data.daily_views_model.uniques;

        let all_time_clones = api_data.daily_clones_model.count;
        let all_time_clones_unique = api_data.daily_clones_model.uniques;

        let weekly_views =
            api_data
                .biweekly_views_model
                .views
                .iter()
                .fold(HashMap::new(), |mut acc, x| {
                    acc.insert(
                        x.timestamp.clone(),
                        QuantifiableEvents {
                            amount: x.count,
                            amount_unique: x.uniques,
                        },
                    );
                    acc
                });

        let weekly_clones =
            api_data
                .biweekly_clones_model
                .clones
                .iter()
                .fold(HashMap::new(), |mut acc, x| {
                    acc.insert(
                        x.timestamp.clone(),
                        QuantifiableEvents {
                            amount: x.count,
                            amount_unique: x.uniques,
                        },
                    );
                    acc
                });

        let hourly_views =
            api_data
                .daily_views_model
                .views
                .iter()
                .fold(HashMap::new(), |mut acc, x| {
                    acc.insert(
                        x.timestamp.clone(),
                        QuantifiableEvents {
                            amount: x.count,
                            amount_unique: x.uniques,
                        },
                    );
                    acc
                });

        let hourly_clones =
            api_data
                .daily_clones_model
                .clones
                .iter()
                .fold(HashMap::new(), |mut acc, x| {
                    acc.insert(
                        x.timestamp.clone(),
                        QuantifiableEvents {
                            amount: x.count,
                            amount_unique: x.uniques,
                        },
                    );
                    acc
                });

        let mut weekly_referrals = HashMap::new();
        let mut all_time_referrals: HashMap<Referrer, QuantifiableEvents> = HashMap::new();

        let mut total_referrals = 0;
        let mut total_referrals_unique = 0;

        for new_referral in &api_data.biweekly_referrals_model {
            let referrals = weekly_referrals
                .entry(new_referral.referrer.clone())
                .or_insert(HashMap::new());

            referrals.insert(
                fourteen_days_ago.clone(),
                QuantifiableEvents {
                    amount: new_referral.count,
                    amount_unique: new_referral.uniques,
                },
            );

            all_time_referrals.insert(
                new_referral.referrer.clone(),
                QuantifiableEvents {
                    amount: new_referral.count,
                    amount_unique: new_referral.uniques,
                },
            );

            total_referrals += new_referral.count;
            total_referrals_unique += new_referral.uniques;
        }

        let mut weekly_content_visits = HashMap::new();
        let mut all_time_content_paths: HashMap<ContentPath, QuantifiableEvents> = HashMap::new();

        let mut total_content_visits = 0;
        let mut total_content_visits_unique = 0;

        for new_content_visit in &api_data.biweekly_content_visits_model {
            let content_visits = weekly_content_visits
                .entry(new_content_visit.path.clone())
                .or_insert(HashMap::new());

            content_visits.insert(
                fourteen_days_ago.clone(),
                QuantifiableEvents {
                    amount: new_content_visit.count,
                    amount_unique: new_content_visit.uniques,
                },
            );

            all_time_content_paths.insert(
                new_content_visit.path.clone(),
                QuantifiableEvents {
                    amount: new_content_visit.count,
                    amount_unique: new_content_visit.uniques,
                },
            );

            total_content_visits += new_content_visit.count;
            total_content_visits_unique += new_content_visit.uniques;
        }

        return Ok(RepositoryTrafficChronicle {
            total_views: all_time_views,
            total_views_unique: all_time_views_unique,
            total_clones: all_time_clones,
            total_clones_unique: all_time_clones_unique,
            all_time_referrals,
            all_time_content_paths,
            total_content_visits,
            total_content_visits_unique,
            weekly_views,
            weekly_clones,
            hourly_views,
            hourly_clones,
            weekly_referrals,
            weekly_content_visits,
            total_referrals,
            total_referrals_unique,
        });
    }
}
