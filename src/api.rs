use anyhow as ah;
use minreq::get;
use serde::de::DeserializeOwned;

use crate::api_models::*;

const API_BASE: &'static str = "https://api.github.com";

typedef!(pub, AuthToken, String);
typedef!(pub, EndpointURL, String);
typedef!(pub, EndpointTemplate, String);
typedef!(pub, URL, String);

fn attempt_api_request<T: DeserializeOwned>(token: &AuthToken, url: &String) -> ah::Result<T> {
    let response = get(url)
        .with_header("Accept", "application/vnd.github+json")
        .with_header("Authorization", format!("Bearer {}", token.0))
        .with_header("X-GitHub-Api-Version", "2022-11-28")
        .with_header("User-Agent", "Awesome-Octocat-App")
        .send()?;

    let status_code: &i32 = &response.status_code;

    match status_code {
        200 => {
            let content = response.as_str()?.to_string();
            let deserialized: T = sj::from_str::<T>(&content).map_err(|e| ah::anyhow!(e))?;
            return Ok(deserialized);
        }
        code => {
            return Err(ah::anyhow!(
                "Request failed with status code {}, {}",
                code,
                response.reason_phrase
            ))
        }
    }
}

macro_rules! define_request_fn {
    ($name:ident, $type:ty, $endpoint:expr) => {
        pub fn $name(token: &AuthToken, author: &String, repo: &String) -> ah::Result<$type> {
            let endpoint = format!($endpoint, API_BASE, author, repo);
            attempt_api_request(token, &endpoint)
        }
    };
}

define_request_fn!(
    request_clones_daily,
    ModelRepoClonesDaily,
    "{}/repos/{}/{}/traffic/clones?per=day"
);

define_request_fn!(
    request_clones_weekly,
    ModelRepoClonesBiWeekly,
    "{}/repos/{}/{}/traffic/clones?per=week"
);

define_request_fn!(
    request_views_daily,
    ModelRepoViewsDaily,
    "{}/repos/{}/{}/traffic/views?per=day"
);

define_request_fn!(
    request_views_weekly,
    ModelRepoViewsBiWeekly,
    "{}/repos/{}/{}/traffic/views?per=week"
);

define_request_fn!(
    request_referrers_weekly,
    ModelReferrers,
    "{}/repos/{}/{}/traffic/popular/referrers"
);

define_request_fn!(
    request_popular_paths_weekly,
    ModelContentTrafficBiWeekly,
    "{}/repos/{}/{}/traffic/popular/paths"
);
