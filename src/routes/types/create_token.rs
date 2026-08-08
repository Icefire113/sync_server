#[derive(serde::Deserialize, Debug)]
pub struct CreateTokenReq {
    pub name: String,
    pub duration_days: u32,
}

#[derive(serde::Serialize, Debug)]
pub struct CreateTokenRes {
    pub token: String,
    pub token_id: i64,
}
