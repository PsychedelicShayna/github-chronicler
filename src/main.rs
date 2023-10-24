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
mod timecalc;
mod api_models;
mod report_models;
mod reporter;

use api::*;
use reporter::*;

use anyhow as ah;

use std::io::Read;

const AUTH_FILE: &str = "./auth.secret";

fn read_token(file: String) -> ah::Result<AuthToken> {
    let mut file = std::fs::OpenOptions::new().read(true).open(file)?;

    let mut str: String = String::new();
    file.read_to_string(&mut str)?;

    Ok(AuthToken(str.trim().to_string()))
}

fn main() -> ah::Result<()> {
    let auth_token = read_token(AUTH_FILE.into())?;

    let author: String = "PsychedelicShayna".into();
    let repo: String = "cursor-locker".into();

    let report_data = request_report_data(&auth_token, &author, &repo)?;
    let file_path: String = format!("{}.{}.report.json", author, repo);

    if !std::path::Path::new(file_path.as_str()).exists() {
        let new_report = generate_new_report(report_data);
        save_report_file(new_report, &file_path)?;
        return Ok(());
    }

    let old_report = load_report_file(&file_path)?;
    let updated_report = update_existing_report(old_report.clone(), &report_data)?;

    save_report_file(updated_report, &file_path)?;

    Ok(())
}
