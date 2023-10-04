use chrono::DateTime;
use serde::{Deserialize, Serialize};

// Struct to hold each notice.
#[derive(Hash, Debug, Serialize, Deserialize, Clone)]
pub struct NoticeElement {
    pub hash: String,
    pub title: String,
    pub date: String,
    pub utc_time: DateTime<chrono::Utc>,
    pub file_url: String,
}

#[derive(Hash, Debug, Serialize, Deserialize, Clone)]
pub struct DbMetaData {
    pub latest_hash: String,
    pub subscribed_users: Vec<String>,
}
