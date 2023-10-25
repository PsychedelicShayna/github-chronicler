pub use serde::{self, Deserialize, Serialize};
pub use serde_json::{self as sj, json};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoClonesHourly {
    pub count: u64,
    pub timestamp: String,
    pub uniques: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoClonesDaily {
    pub clones: Vec<ModelRepoClonesHourly>,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoClonesWeekly {
    pub count: u64,
    pub timestamp: String,
    pub uniques: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoClonesBiWeekly {
    pub clones: Vec<ModelRepoClonesWeekly>,
    pub count: u64,
    pub uniques: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelReferrer {
    pub count: u64,
    pub referrer: String,
    pub uniques: u64,
}

pub type ModelReferrerals = Vec<ModelReferrer>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelContentTraffic {
    pub count: u64,
    pub path: String,
    pub title: String,
    pub uniques: u64,
}

pub type ModelContentTrafficBiWeekly = Vec<ModelContentTraffic>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoViewsHourly {
    pub count: u64,
    pub timestamp: String,
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
    pub count: u64,
    pub timestamp: String,
    pub uniques: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoViewsBiWeekly {
    pub count: u64,
    pub uniques: u64,
    pub views: Vec<ModelRepoViewsWeekly>,
}

pub type ModelRepoStargazers = Vec<ModelRepoStargazer>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoStargazer {
    pub avatar_url: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: u64,
    pub login: String,
    pub node_id: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub request_watchers: String,
    pub r#type: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub url: String,
}

pub type ModelRepoWatchers = Vec<ModelRepoWatcher>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoWatcher {
    pub avatar_url: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: u64,
    pub login: String,
    pub node_id: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub r#type: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub url: String,
}

pub type ModelRepoForks = Vec<ModelRepoFork>;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelRepoFork {
  pub allow_forking: bool,
  pub archived: bool,
  pub archive_url: String,
  pub assignees_url: String,
  pub blobs_url: String,
  pub branches_url: String,
  pub clone_url: String,
  pub collaborators_url: String,
  pub comments_url: String,
  pub commits_url: String,
  pub compare_url: String,
  pub contents_url: String,
  pub contributors_url: String,
  pub created_at: String,
  pub default_branch: String,
  pub deployments_url: String,
  pub description: String,
  pub disabled: bool,
  pub downloads_url: String,
  pub events_url: String,
  pub fork: bool,
  pub forks_count: u64,
  pub forks: u64,
  pub forks_url: String,
  pub full_name: String,
  pub git_commits_url: String,
  pub git_refs_url: String,
  pub git_tags_url: String,
  pub git_url: String,
  pub has_discussions: bool,
  pub has_downloads: bool,
  pub has_issues: bool,
  pub has_pages: bool,
  pub has_projects: bool,
  pub has_wiki: bool,
  pub homepage: String,
  pub hooks_url: String,
  pub html_url: String,
  pub id: u64,
  pub issue_comment_url: String,
  pub issue_events_url: String,
  pub issues_url: String,
  pub is_template: bool,
  pub keys_url: String,
  pub labels_url: String,
  pub language: Option<String>,
  pub languages_url: String,
  pub license: ModelLicense,
  pub merges_url: String,
  pub milestones_url: String,
  pub mirror_url: Option<String>,
  pub name: String,
  pub node_id: String,
  pub notifications_url: String,
  pub open_issues_count: u64,
  pub open_issues: u64,
  pub owner: OwnerModel,
  pub permissions: ModelPermissions,
  pub private: bool,
  pub pulls_url: String,
  pub pushed_at: String,
  pub releases_url: String,
  pub size: u64,
  pub ssh_url: String,
  pub stargazers_count: u64,
  pub stargazers_url: String,
  pub statuses_url: String,
  pub subscribers_url: String,
  pub subscription_url: String,
  pub svn_url: String,
  pub tags_url: String,
  pub teams_url: String,
  pub topics: Vec<String>,
  pub trees_url: String,
  pub updated_at: String,
  pub url: String,
  pub visibility: String,
  pub watchers_count: u64,
  pub watchers: u64,
  pub web_commit_signoff_required: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelLicense {
    key: String,
    name: String,
    node_id: String,
    spdx_id: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct ModelPermissions {
    admin: bool,
    maintain: bool,
    pull: bool,
    push: bool,
    triage: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct OwnerModel {
    avatar_url: String,
    events_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    gravatar_id: String,
    html_url: String,
    id: u64,
    login: String,
    node_id: String,
    organizations_url: String,
    received_events_url: String,
    repos_url: String,
    r#type: String,
    site_admin: bool,
    starred_url: String,
    subscriptions_url: String,
    url: String,
}
