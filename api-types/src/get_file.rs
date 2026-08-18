use chrono::{DateTime, Utc};
use uuid::Uuid;

// No request, file id is in path param

#[derive(serde::Serialize, Debug)]
pub struct GetFileInfoRes {
    pub id: Uuid,
    pub name: String,
    pub hash: u64,
    pub last_updated_from: String,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
