#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct CreateUserReq {
    pub username: String,
    pub password: String,
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct CreateUserRes {
    pub id: i64,
    pub username: String,
    pub access_token: String,
}
