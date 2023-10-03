use serde::{Deserialize, Serialize};

// Struct to hold each notice.
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct NoticeElement {
    pub serial_number: String,
    pub hash: String,
    pub title: String,
    pub date: String,
    pub file_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbDomData {
    latest_hash: String,
    subscribed_users: Vec<String>,
    notices: Vec<NoticeElement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbCollectionType {
    data: DbDomData,
    data_id: String,
}
