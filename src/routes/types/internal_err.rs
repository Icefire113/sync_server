use std::fmt::Display;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct InternalErrorRes {
    pub code: String,
    pub error_detail: Option<String>,
}

impl InternalErrorRes {
    fn new(code: InternalErrorCode) -> Self {
        Self {
            code: code.to_string(),
            error_detail: match code {
                InternalErrorCode::NoSuchUserFound => Some("User not found".to_string()),
                InternalErrorCode::UsernameTooShort => Some("Username too short".to_string()),
                InternalErrorCode::UsernameTooLong => Some("Username too long".to_string()),
                InternalErrorCode::FileAlreadyExists => Some("File already exists".to_string()),
                InternalErrorCode::UsernameTaken => Some("Username taken".to_string()),
                InternalErrorCode::Unauthorized => Some("Unauthorized".to_string()),
                InternalErrorCode::AccountNotEnabled => Some("Account not enabled".to_string()),
                InternalErrorCode::TokenExpired => Some("Token expired".to_string()),
                InternalErrorCode::TokenRevoked => Some("Token revoked".to_string()),
                InternalErrorCode::TokenNotFound => Some("Token not found".to_string()),
                InternalErrorCode::BadRequest => Some("Bad request".to_string()),
                InternalErrorCode::InternalError => Some("Internal error".to_string()),
                InternalErrorCode::Forbidden => Some("Forbidden".to_string()),
                InternalErrorCode::CannotRevokeAllTokens => {
                    Some("You cannot revoke your last valid token".to_string())
                }
                InternalErrorCode::TokenAlreadyRevoked => Some("Token already revoked".to_string()),
                InternalErrorCode::InvalidUsernameOrPassword => {
                    Some("Invalid username or password".to_string())
                }
                InternalErrorCode::UsernameContainsInvalidChars => {
                    Some("Username contains invalid characters".to_string())
                }
                _ => None,
            },
        }
    }
}

impl From<InternalErrorCode> for InternalErrorRes {
    fn from(value: InternalErrorCode) -> Self {
        Self::new(value)
    }
}

pub enum InternalErrorCode {
    NoSuchUserFound,
    InternalDBError,
    InternalError,
    PasswordHash,
    UsernameTooShort,
    UsernameTooLong,
    FileAlreadyExists,
    UsernameTaken,
    Unauthorized,
    TokenExpired,
    TokenRevoked,
    AccountNotEnabled,
    BadRequest,
    TokenNotFound,
    Forbidden,
    CannotRevokeAllTokens,
    TokenAlreadyRevoked,
    InvalidUsernameOrPassword,
    HashPasswordVerify,
    InsufficientRole,
    UsernameContainsInvalidChars,
}

impl Display for InternalErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSuchUserFound => write!(f, "E0001"),
            Self::InternalDBError => write!(f, "E0002"),
            Self::PasswordHash => write!(f, "E0003"),
            Self::UsernameTooShort => write!(f, "E0004"),
            Self::UsernameTooLong => write!(f, "E0005"),
            Self::FileAlreadyExists => write!(f, "E0006"),
            Self::UsernameTaken => write!(f, "E0007"),
            Self::Unauthorized => write!(f, "E0008"),
            Self::AccountNotEnabled => write!(f, "E0009"),
            Self::BadRequest => write!(f, "E0010"),
            Self::TokenExpired => write!(f, "E0011"),
            Self::TokenRevoked => write!(f, "E0012"),
            Self::InternalError => write!(f, "E0013"),
            Self::TokenNotFound => write!(f, "E0014"),
            Self::Forbidden => write!(f, "E0015"),
            Self::CannotRevokeAllTokens => write!(f, "E0016"),
            Self::TokenAlreadyRevoked => write!(f, "E0017"),
            Self::InvalidUsernameOrPassword => write!(f, "E0018"),
            Self::HashPasswordVerify => write!(f, "E0019"),
            Self::InsufficientRole => write!(f, "E0020"),
            Self::UsernameContainsInvalidChars => write!(f, "E0021"),
        }
    }
}
