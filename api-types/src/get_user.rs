// No request, info is in path param

use chrono::{DateTime, Utc};

use crate::role::Role;

#[derive(serde::Serialize, Debug)]
pub struct GetUserRes {
    pub id: i64,
    pub username: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub enabled: bool,
}
