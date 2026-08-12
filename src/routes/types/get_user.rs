// No request, info is in path param

use chrono::{DateTime, Utc};
use entity::user;

#[derive(serde::Serialize, Debug)]
pub struct GetUserRes {
    pub id: i64,
    pub username: String,
    pub role: user::Role,
    pub created_at: DateTime<Utc>,
    pub enabled: bool,
}
