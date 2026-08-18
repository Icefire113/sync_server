use chrono::{DateTime, Utc};

use crate::role::Role;

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    // NOTE: if we allow user to change their username, we need to make sure that the username follows the same rules as when creating a new user
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateUserRes {
    pub id: i64,
    pub role: Role,
    pub username: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}
