#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct RevokeTokenReq {
    pub id: i64,
}

// No response, returns 204
