#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct RequestTokenReq {
    pub username: String,
    pub password: String,
    pub token_name: String,
    pub duration_days: u32,
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct RequestTokenRes {
    pub token: String,
    pub token_id: i64,
}
