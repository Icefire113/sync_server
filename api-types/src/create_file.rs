use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct CreateFileReq {
    pub name: String,
    pub hash: u64,
    pub file_bytes: Vec<u8>,
    /// The machine that is creating this file
    pub machine_name: String,
}

#[derive(serde::Serialize, Debug)]
pub struct CreateFileRes {
    pub id: Uuid,
}
