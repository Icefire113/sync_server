use chrono::{DateTime, Utc};

use crate::role::Role;

#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct UpdateUserReq {
    #[cfg_attr(feature = "client", serde(skip_serializing_if = "Option::is_none"))]
    pub enabled: Option<bool>,
    #[cfg_attr(feature = "client", serde(skip_serializing_if = "Option::is_none"))]
    pub role: Option<Role>,
    // NOTE: if we allow user to change their username, we need to make sure that the username follows the same rules as when creating a new user
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct UpdateUserRes {
    pub id: i64,
    pub role: Role,
    pub username: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}
