use chrono::{DateTime, Utc};

use crate::role::Role;

// No request, info is in path param

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct GetUserRes {
    pub id: i64,
    pub username: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub enabled: bool,
}
