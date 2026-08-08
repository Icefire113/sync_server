#[derive(serde::Deserialize, Debug)]
pub struct CreateUserReq {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize, Debug)]
pub struct CreateUserRes {
    pub username: String,
    pub access_token: String,
}
