use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub body_raw: String,
    pub body_cooked: String,
    pub created: DateTime<Utc>,
    pub username: String,
    pub url: String,
    pub source_id: String,
}
