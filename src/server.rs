use crate::api::{self, AuthToken, Endpoint, EndpointTemplate, EndpointURL, RepoTraffic, URL};
use anyhow as ah;

use serde::{self, Deserialize, Serialize};
use serde_json::{self as sj, json};

struct Server {
    auth_token: AuthToken,
    username: String,
    // config: ServerConfig,
}


impl Server {
    fn monitor_loop() {}
    fn setup() {}
    fn generate_report() {}
    fn save_report() {}
}
