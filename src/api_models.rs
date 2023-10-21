pub use serde::{self, Deserialize, Serialize};
pub use serde_json::{self as sj, json};

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelRepoClonesHourly {
    pub timestamp: String,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelRepoClonesDaily {
    pub count: u64,
    pub uniques: u64,
    pub clones: Vec<ModelRepoClonesHourly>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelRepoClonesWeekly {
    pub timestamp: String,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelRepoClonesBiWeekly {
    pub count: u64,
    pub uniques: u64,
    pub clones: Vec<ModelRepoClonesWeekly>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelReferrer {
    pub referrer: String,
    pub count: u64,
    pub uniques: u64,
}

pub type ModelReferrerals = Vec<ModelReferrer>;

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelContentTraffic {
    pub path: String,
    pub title: String,
    pub count: u64,
    pub uniques: u64,
}

pub type ModelContentTrafficBiWeekly = Vec<ModelContentTraffic>;

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelRepoViewsHourly {
    pub timestamp: String,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelRepoViewsDaily {
    pub count: u64,
    pub uniques: u64,
    pub views: Vec<ModelRepoViewsHourly>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelRepoViewsWeekly {
    pub timestamp: String,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ModelRepoViewsBiWeekly {
    pub count: u64,
    pub uniques: u64,
    pub views: Vec<ModelRepoViewsWeekly>,
}
