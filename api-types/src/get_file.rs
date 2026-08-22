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

// No request, file id is in path param

pub type GetFileContentRes = Vec<u8>;

#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[derive(Debug)]
pub struct GetAllFilesReq {
    pub include_deleted: bool,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[cfg(feature = "client")]
impl Default for GetAllFilesReq {
    fn default() -> Self {
        Self {
            include_deleted: false,
            limit: Some(100),
            offset: Some(0),
        }
    }
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct GetAllFilesRes {
    pub files: Vec<PartialFileInfo>,
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct PartialFileInfo {
    pub id: Uuid,
    pub name: String,
    pub hash: u64,
}
