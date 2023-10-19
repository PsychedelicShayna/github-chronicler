pub use serde::{self, Deserialize, Serialize};
pub use serde_json::{self as sj, json};

#[derive(Deserialize, Debug)]
pub struct ModelRepoClonesHourly {
    tmestamp: String,
    count: u64,
    uniques: u64,
}

#[derive(Deserialize, Debug)]
pub struct ModelRepoClonesDaily {
    count: u64,
    uniques: u64,
    clones: Vec<ModelRepoClonesHourly>,
}

#[derive(Deserialize, Debug)]
pub struct ModelRepoClonesWeekly {
    timestamp: String,
    count: u64,
    uniques: u64,
}

#[derive(Deserialize, Debug)]
pub struct ModelRepoClonesBiWeekly {
    count: u64,
    uniques: u64,
    clones: Vec<ModelRepoClonesWeekly>,
}

#[derive(Deserialize, Debug)]
pub struct ModelReferrers {
    referrer: String,
    count: u64,
    uniques: u64,
}

type ModelPopularReferrersBiWeekly = Vec<ModelReferrers>;

#[derive(Deserialize, Debug)]
pub struct ModelPopularPaths {
    paths: String,
    title: String,
    count: u64,
    uniques: u64,
}

type ModelPopularPathsBiWeekly = Vec<ModelPopularPaths>;

#[derive(Deserialize, Debug)]
pub struct ModelRepoViewsHourly {
    timestamp: String,
    count: u64,
    uniques: u64,
}

#[derive(Deserialize, Debug)]
pub struct ModelRepoViewsDaily {
    count: u64,
    uniques: u64,
    views: Vec<ModelRepoViewsHourly>,
}

#[derive(Deserialize, Debug)]
pub struct ModelRepoViewsWeekly {
    timestamp: String,
    count: u64,
    uniques: u64,
}

#[derive(Deserialize, Debug)]
pub struct ModelRepoViewsBiWeekly {
    count: u64,
    uniques: u64,
    views: Vec<ModelRepoViewsWeekly>,
}
