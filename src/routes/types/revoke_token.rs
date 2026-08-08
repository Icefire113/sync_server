#[derive(serde::Deserialize, Debug)]
pub struct RevokeTokenReq {
    pub id: i64,
}

#[derive(serde::Serialize, Debug)]
pub struct RevokeTokenRes {}
