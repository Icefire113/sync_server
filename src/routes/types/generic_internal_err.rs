use std::fmt::Display;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct InternalErrorRes {
    /// The internal error code, we should not tell the user what these mean
    /// TODO: Move to enum
    pub code: String,
    pub error_detail: Option<String>,
}

impl InternalErrorRes {
    pub fn new(code: InternalErrorCode) -> Self {
        Self {
            code: code.to_string(),
            error_detail: match code {
                InternalErrorCode::NoSuchUserFoundError => Some("User not found".to_string()),
                InternalErrorCode::UsernameTooShort => Some("Username too short".to_string()),
                InternalErrorCode::UsernameTooLong => Some("Username too long".to_string()),
                InternalErrorCode::FileAlreadyExists => Some("File already exists".to_string()),
                InternalErrorCode::UsernameTaken => Some("Username taken".to_string()),
                _ => None,
            },
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub enum InternalErrorCode {
    NoSuchUserFoundError,
    InternalDBError,
    AccessKeyHashError,
    UsernameTooShort,
    UsernameTooLong,
    FileAlreadyExists,
    UsernameTaken,
}

impl Display for InternalErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InternalErrorCode::NoSuchUserFoundError => {
                write!(f, "E0001")
            }
            InternalErrorCode::InternalDBError => {
                write!(f, "E0002")
            }
            InternalErrorCode::AccessKeyHashError => {
                write!(f, "E0003")
            }
            InternalErrorCode::UsernameTooShort => {
                write!(f, "E0004")
            }
            InternalErrorCode::UsernameTooLong => {
                write!(f, "E0005")
            }
            InternalErrorCode::FileAlreadyExists => {
                write!(f, "E0006")
            }
            InternalErrorCode::UsernameTaken => {
                write!(f, "E0007")
            }
        }
    }
}
