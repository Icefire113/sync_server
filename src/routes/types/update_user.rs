use chrono::{DateTime, Utc};

use crate::db::schema::user;

#[derive(Debug, serde::Deserialize)]
pub struct UpdateUserReq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<user::Role>,
}

#[derive(Debug, serde::Serialize)]
pub struct UpdateUserRes {
    pub id: i64,
    pub role: user::Role,
    pub username: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}
