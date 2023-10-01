use serde::{Deserialize, Serialize};

// Struct to hold each notice.
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct NoticeElement {
    pub serial_number: String,
    pub hash: u64,
    pub title: String,
    pub date: String,
    pub file_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbDomData {
    latest_hash: String,
    subscribed_users: Vec<String>,
    notices: NoticeElement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbCollectionType {
    data: DbDomData,
    data_id: String,
}
