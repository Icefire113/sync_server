#[derive(serde::Deserialize, Debug)]
pub struct RevokeTokenReq {
    pub id: i64,
}

// No response, returns 204
