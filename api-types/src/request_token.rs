#[derive(serde::Deserialize, Debug)]
pub struct RequestTokenReq {
    pub username: String,
    pub password: String,
    pub token_name: String,
    pub duration_days: u32,
}

#[derive(serde::Serialize, Debug)]
pub struct RequestTokenRes {
    pub token: String,
    pub token_id: i64,
}
