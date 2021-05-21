use chrono::prelude::*;

pub fn now() -> String {
    Utc::now().to_rfc2822()
}
