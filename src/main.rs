use api::Endpoint;

macro_rules! typedef {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone)]
        struct $name(pub $type);

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

    ($vis:vis, $name:ident, $type:ty) => {
        #[derive(Debug, Clone)]
        $vis struct $name(pub $type);

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

mod api;
mod api_models;
mod server;

const STOKENN: &'static str = "!$1!";
const STOKENR: &'static str = "!$2!";

use api::*;
use api_models::*;
use std::io::{BufReader, BufWriter, Read, Write};

fn read_token(file: String) -> AuthToken {
    let mut file = std::fs::OpenOptions::new().read(true).open(file).unwrap();

    let mut str: String = String::new();
    file.read_to_string(&mut str).unwrap();

    return AuthToken(str);
}

fn main() {
    let endpoint = api::RepoTraffic::Views;

    let t = read_token("./auth".into());
    println!("Token: {}", t.0);

    let response = endpoint.send(t, |templ| {
        let mut s = String::from(templ);
        s = s.replace(STOKENN, "PsychedelicShayna");
        s = s.replace(STOKENR, "cursor-locker");
        EndpointURL::from(s)
    });

    match response {
        Ok(r) => {
            let code = r.status_code;
            let content = r.as_str().unwrap().to_string();
            println!("Response {}\n,Content:\n{}", code, content);
            let dds: ModelRepoViewsDaily = serde_json::from_str(content.as_str()).unwrap();
            println!("{:?}", dds);
        }

        Err(e) => {
            eprintln!("{:?}", e);
            panic!()
        }
    }
}
