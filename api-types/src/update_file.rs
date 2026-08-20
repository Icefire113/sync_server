use chrono::{DateTime, Utc};

#[derive(Debug, serde::Deserialize)]
pub struct UpdateContents {
    pub file_bytes: Vec<u8>,
    pub expected_hash: u64,
    pub hash: u64,
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateFileReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machine_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_contents: Option<UpdateContents>,
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateFileRes {
    pub name: String,
    pub hash: u64,
    pub machine_name: String,
    pub updated_at: DateTime<Utc>,
}
