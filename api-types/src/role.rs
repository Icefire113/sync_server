#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "PascalCase")]
pub enum Role {
    Banned = 0,
    #[default]
    User = 1,
    Admin = 2,
}
