use crate::domain::errors::my_error::MyError;
use regex::Regex;
use chrono::{self, Utc};

pub fn get_after_dashdash(s: &str) -> Option<&str> {
    let mut parts = s.split("--");
    parts.next()?;  // consume everything before "--"
    parts.next()    // get everything after "--"
}

pub fn process_globe_id(globe_id: &str) -> Result<String, MyError> {
    let globe_id = globe_id.to_lowercase();

    if globe_id.len() > 12 {
        return Err(MyError::ValidationError("globe_id should not be longer than 12 characters".to_string()));
    }

    let re = Regex::new(r"^[a-z1-9_]+$").unwrap();
    if !re.is_match(&globe_id) {
        return Err(MyError::ValidationError("globe_id contains invalid characters".to_string()));
    }

    Ok(globe_id)
}

pub fn generate_timestamp() -> String {
    let now = Utc::now();
    (now.timestamp_subsec_nanos() as i64 + now.timestamp() * 1_000_000_000).to_string()
}

