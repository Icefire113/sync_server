use chrono::{DateTime, Utc};

#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct UpdateContents {
    pub file_bytes: Vec<u8>,
    pub expected_hash: u64,
    pub hash: u64,
}

#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct UpdateFileReq {
    #[cfg_attr(feature = "client", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,
    pub machine_name: String,
    #[cfg_attr(feature = "client", serde(skip_serializing_if = "Option::is_none"))]
    pub update_contents: Option<UpdateContents>,
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct UpdateFileRes {
    pub name: String,
    pub hash: u64,
    pub machine_name: String,
    pub updated_at: DateTime<Utc>,
}
