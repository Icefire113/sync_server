use chrono::{DateTime, Utc};

#[derive(serde::Deserialize, Debug)]
pub struct GetTokenInfoReq {
    pub token: String,
}

#[derive(serde::Serialize, Debug)]
pub struct GetTokenInfoRes {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
}
