use uuid::Uuid;

#[cfg_attr(feature = "client", derive(serde::Serialize))]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[derive(Debug)]
pub struct CreateFileReq {
    pub name: String,
    pub hash: u64,
    pub file_bytes: Vec<u8>,
    /// The machine that is creating this file
    pub machine_name: String,
}

#[cfg_attr(feature = "client", derive(serde::Deserialize))]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[derive(Debug)]
pub struct CreateFileRes {
    pub id: Uuid,
}
