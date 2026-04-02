#[derive(serde::Deserialize, Debug)]
pub struct GetAllDiscriminatorsReq {
    pub username: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetAllDiscriminatorsRes {
    pub discriminators: Vec<String>,
}
