use anyhow as ah;
use minreq::{self, get, Request, Response};

const API_BASE: &'static str = "https://api.github.com";

typedef!(pub, AuthToken, String);
typedef!(pub, EndpointURL, String);
typedef!(pub, EndpointTemplate, String);
typedef!(pub, URL, String);

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

pub trait Endpoint
where
    Self: Into<EndpointTemplate>,
{
    fn send<F: FnOnce(EndpointTemplate) -> EndpointURL>(
        &self,
        token: AuthToken,
        func: F,
    ) -> ah::Result<Response>;
}

#[derive(Debug, Clone)]
pub enum RepoTraffic {
    Clones,
    ReferredPaths,
    RefferalSources,
    Views,
}

impl From<RepoTraffic> for EndpointTemplate {
    fn from(value: RepoTraffic) -> Self {
        match value {
            RepoTraffic::Clones => {
                format!("{}/repos/{}/{}/traffic/clones", API_BASE, "!$1!", "!$2!")
            }
            RepoTraffic::Views => format!("{}/repos/{}/{}/traffic/views", API_BASE, "!$1!", "!$2!"),
            RepoTraffic::ReferredPaths => {
                format!(
                    "{}/repos/{}/{}/traffic/popular/paths",
                    API_BASE, "!$1!", "!$2!"
                )
            }
            RepoTraffic::RefferalSources => format!(
                "{}/repos/{}/{}/traffic/popular/referrers",
                API_BASE, "!$1!", "!$2!"
            ),
        }
        .into()
    }
}

impl Endpoint for RepoTraffic {
    fn send<F: FnOnce(EndpointTemplate) -> EndpointURL>(
        &self,
        token: AuthToken,
        func: F,
    ) -> ah::Result<Response> {
        let template: EndpointTemplate = EndpointTemplate::from(self.clone());
        let endpoint_url = func(template);
        Ok(create_request(endpoint_url, token).send()?)
    }
}

fn create_request(url: EndpointURL, token: AuthToken) -> Request {
    get(url.0)
        .with_header("Accept", "application/vnd.github+json")
        .with_header("Authorization", format!("Bearer {}", token.0))
        .with_header("X-GitHub-Api-Version", "2022-11-28")
        .with_header("User-Agent", "Awesome-Octocat-App")
}
