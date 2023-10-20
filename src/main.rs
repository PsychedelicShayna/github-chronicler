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
mod report_models;
mod reporter;

use api::*;
use reporter::*;

use anyhow as ah;

use std::io::Read;

fn read_token(file: String) -> AuthToken {
    let mut file = std::fs::OpenOptions::new().read(true).open(file).unwrap();
    let mut str: String = String::new();
    file.read_to_string(&mut str).unwrap();

    AuthToken(str)
}

fn main() -> ah::Result<()> {
    let token = read_token("./auth".into());
    let author: String = "PsychedelicShayna".into();
    let repo: String = "cursor-locker".into();

    let report_data = request_report_data(&token, &author, &repo)?;
    let report = create_new_report(report_data);

    let report_file: String = format!("{}.{}.report.json", author, repo);
    println!("{:?}", report);

    save_report_file(report, &report_file)?;

    Ok(())
}
