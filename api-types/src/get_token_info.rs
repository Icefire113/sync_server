use chrono::{DateTime, Utc};

#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct GetTokenInfoReq {
    pub token: String,
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct GetTokenInfoRes {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub last_used: Option<DateTime<Utc>>,
}
