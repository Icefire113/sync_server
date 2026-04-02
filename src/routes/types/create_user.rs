#[derive(serde::Deserialize, Debug)]
pub struct CreateUserReq {
    pub username: String,
}

#[derive(serde::Serialize, Debug)]
pub struct CreateUserRes {
    pub username: String,
    pub access_key: String,
}
