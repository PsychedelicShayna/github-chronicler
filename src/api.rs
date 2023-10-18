use anyhow as ah;
use minreq::{self, get, Request, Response};

// Will be moved to main.rs
macro_rules! typedef {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone)]
        struct $name($type);
        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                $name(value)
            }
        }

        impl From<$name> for $type {
            fn from(value: $name) -> Self {
                value.0
            }
        }
    };
}

const API_BASE: &'static str = "https://api.github.com";

typedef!(AuthToken, String);
typedef!(EndpointURL, String);
typedef!(EndpointTemplate, String);
typedef!(URL, String);

impl From<&str> for EndpointURL {
    fn from(value: &str) -> Self {
        EndpointURL(value.into())
    }
}

impl From<&str> for EndpointTemplate {
    fn from(value: &str) -> Self {
        EndpointTemplate(value.into())
    }
}

impl From<&str> for URL {
    fn from(value: &str) -> Self {
        URL(value.into())
    }
}

trait Endpoint
where
    Self: Into<EndpointTemplate>,
{
    fn fill<F: FnOnce(EndpointTemplate) -> EndpointURL>(&self, func: F) -> EndpointURL;
    fn send(&self, eurl: EndpointURL) -> ah::Result<Response>;
}

#[derive(Debug, Clone)]
pub enum RepoTraffic {
    Clones,
    ReferredPaths,
    RefferalSources,
    Views,
}

impl From<&RepoTraffic> for EndpointTemplate {
    fn from(value: &RepoTraffic) -> Self {
        match value {
            RepoTraffic::Clones => format!("{}/repos/{}/{}/traffic/clones", API_BASE, "{}", "{}"),
            RepoTraffic::Views => format!("{}/repos/{}/{}/traffic/views", API_BASE, "{}", "{}"),
            RepoTraffic::ReferredPaths => {
                format!("{}/repos/{}/{}/traffic/popular/paths", API_BASE, "{}", "{}")
            }
            RepoTraffic::RefferalSources => format!(
                "{}/repos/{}/{}/traffic/popular/referrers",
                API_BASE, "{}", "{}"
            ),
        }
        .into()
    }
}

impl Endpoint for &RepoTraffic {
    fn send(&self, endpoint_url: EndpointURL) -> ah::Result<Response> {
        let token = AuthToken(String::default());
        Ok(create_request(endpoint_url, token).send()?)
    }

    fn fill<F: FnOnce(EndpointTemplate) -> EndpointURL>(&self, func: F) -> EndpointURL {
        let template: EndpointTemplate = EndpointTemplate::from(*self);
        func(template)
    }
}

fn create_request(url: EndpointURL, token: AuthToken) -> Request {
    get(url.0)
        .with_header("Accept", "application/vnd.github+json")
        .with_header("Authorization", format!("Bearer {}", token.0))
        .with_header("X-GitHub-Api-Version", "2022-11-28")
        .with_header("User-Agent", "Awesome-Octocat-App")
}
