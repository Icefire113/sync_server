#[cfg(feature = "server")]
pub mod extract;

pub mod create_file;
pub mod create_token;
pub mod create_user;
pub mod delete_file;
pub mod delete_user;
pub mod get_file;
pub mod get_token_info;
pub mod get_user;
pub mod internal_err;
pub mod request_token;
pub mod revoke_token;
pub mod role;
pub mod update_file;
pub mod update_user;

use std::fmt::Display;

use http::StatusCode;

use crate::internal_err::{InternalErrorCode, InternalErrorRes};

pub use role::Role;

/// The API response type for the entire api, either a success or an error with a status code and either a typed response, or an error response
pub type ApiResponse<T> = Result<(StatusCode, T), ApiError>;

/// The API error type
#[derive(Debug, Clone)]
pub struct ApiError(pub StatusCode, pub InternalErrorRes);

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<(StatusCode, InternalErrorRes)> for ApiError {
    fn from((status, res): (StatusCode, InternalErrorRes)) -> Self {
        Self(status, res)
    }
}

impl From<(StatusCode, InternalErrorCode)> for ApiError {
    fn from((status, code): (StatusCode, InternalErrorCode)) -> Self {
        Self(status, code.into())
    }
}

#[cfg(feature = "server")]
impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}
