use chrono::{DateTime, Utc};
use uuid::Uuid;

// No request, file id is in path param

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct GetFileInfoRes {
    pub id: Uuid,
    pub name: String,
    pub hash: u64,
    pub last_updated_from: String,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub type GetFileContentRes = Vec<u8>;
