use std::fmt::Display;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct InternalErrorRes {
    /// The internal error code, we should not tell the user what these mean
    /// TODO: Move to enum
    pub code: String,
    pub error_detail: Option<String>,
}

impl InternalErrorRes {
    pub fn new(code: InternalErrorCodes) -> Self {
        Self {
            code: code.to_string(),
            error_detail: match code {
                InternalErrorCodes::NoSuchUserFoundError => Some("User not found".to_string()),
                _ => None,
            },
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum InternalErrorCodes {
    NoSuchUserFoundError,
    InternalDBError,
    PasswordHashError,
}

impl Display for InternalErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalErrorCodes::NoSuchUserFoundError => {
                write!(f, "E0001")
            }
            InternalErrorCodes::InternalDBError => {
                write!(f, "E0002")
            }
            InternalErrorCodes::PasswordHashError => {
                write!(f, "E0003")
            }
        }
    }
}
