pub use serde::{self, Deserialize, Serialize};
pub use serde_json::{self as sj, json};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoClonesHourly {
    pub timestamp: String,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoClonesDaily {
    pub count: u64,
    pub uniques: u64,
    pub clones: Vec<ModelRepoClonesHourly>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoClonesWeekly {
    pub timestamp: String,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoClonesBiWeekly {
    pub count: u64,
    pub uniques: u64,
    pub clones: Vec<ModelRepoClonesWeekly>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelReferrer {
    pub referrer: String,
    pub count: u64,
    pub uniques: u64,
}

pub type ModelReferrerals = Vec<ModelReferrer>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelContentTraffic {
    pub path: String,
    pub title: String,
    pub count: u64,
    pub uniques: u64,
}

pub type ModelContentTrafficBiWeekly = Vec<ModelContentTraffic>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoViewsHourly {
    pub timestamp: String,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoViewsDaily {
    pub count: u64,
    pub uniques: u64,
    pub views: Vec<ModelRepoViewsHourly>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoViewsWeekly {
    pub timestamp: String,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoViewsBiWeekly {
    pub count: u64,
    pub uniques: u64,
    pub views: Vec<ModelRepoViewsWeekly>,
}
