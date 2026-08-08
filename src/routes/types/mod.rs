use axum::{Json, http::StatusCode};

use crate::routes::types::internal_err::InternalErrorRes;

pub mod create_token;
pub mod create_user;
pub mod internal_err;
pub mod request_token;
pub mod revoke_token;

/// The API response type for the entire api, either a success or an error with a status code and either a typed response, or an error response
pub type ApiResponse<T> = Result<(StatusCode, T), ApiError>;

/// The API error type
pub type ApiError = (StatusCode, Json<InternalErrorRes>);
