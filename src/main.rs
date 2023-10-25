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
mod crypto;
mod report;
mod timecalc;
mod watcher;

use api::*;
use api_models::*;
use report::*;

use anyhow as ah;
use std::io::Read;

const HELP_TEXT: &str = "
\\ Hello World
\\ Test";

const AUTH_FILE: &str = "./auth.secret";

fn read_token(file: String) -> ah::Result<AuthToken> {
    let mut file = std::fs::OpenOptions::new().read(true).open(file)?;

    let mut str: String = String::new();
    file.read_to_string(&mut str)?;

    Ok(AuthToken(str.trim().to_string()))
}

fn main() -> ah::Result<()> {
    for (carg, narg) in std::env::args().zip(std::env::args().skip(1)) {
        match (&carg.as_str(), &narg.as_str()) {
            (&"--help" | &"-h", _) => {
                println!("{}", HELP_TEXT);
            }

            (_, _) => return Ok(()),
        }
    }

    let auth_token = read_token(AUTH_FILE.into())?;

    let author: String = "PsychedelicShayna".into();
    let repository: String = "cursor-locker".into();

    // let report = RepositoryReport::request_new(&auth_token, &author, &repository)?;

    let d = request_stargazers(&auth_token, &author, &repository)?;
    for s in d {
        println!("{:?}", s.login);
    }

    // report.save_json_file("./report.json");
    // let loaded_report = RepositoryReport::load_json_file("./report.json")?;
    // loaded_report.save_json_file("./report2.json");

    Ok(())
}
