use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDiscrimReq {
    /// TODO: Move this into the JWT
    pub username: String,
    pub file: SyncedFile,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SyncedFile {
    pub id: Uuid,
    pub file_path_per_machine: HashMap<String, PathBuf>,
    pub custom_name: Option<String>,
    pub file_state: FileState,
    pub file_blob: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileState {
    pub hash: u64,
    pub file_last_modified: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDiscrimRes {}
