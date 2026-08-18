use axum::{
    extract::rejection::{JsonRejection, PathRejection, QueryRejection},
    http::StatusCode,
};

use crate::{ApiError, internal_err::InternalErrorCode};

fn err(status: StatusCode, code: InternalErrorCode) -> ApiError {
    ApiError(status, code.into())
}

impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        let (status, detail) = match rejection {
            JsonRejection::JsonDataError(inner) => (StatusCode::BAD_REQUEST, inner.body_text()),
            JsonRejection::JsonSyntaxError(inner) => (StatusCode::BAD_REQUEST, inner.body_text()),
            JsonRejection::MissingJsonContentType(_) => (
                StatusCode::BAD_REQUEST,
                "Expected Content-Type: application/json".to_string(),
            ),
            other => (other.status(), other.body_text()),
        };
        err(status, InternalErrorCode::JsonRejection(detail))
    }
}

impl From<PathRejection> for ApiError {
    fn from(rejection: PathRejection) -> Self {
        match rejection {
            PathRejection::FailedToDeserializePathParams(inner) => err(
                StatusCode::BAD_REQUEST,
                InternalErrorCode::PathRejection(inner.body_text()),
            ),
            other => err(
                other.status(),
                InternalErrorCode::PathRejection(other.body_text()),
            ),
        }
    }
}

impl From<QueryRejection> for ApiError {
    fn from(rejection: QueryRejection) -> Self {
        match rejection {
            QueryRejection::FailedToDeserializeQueryString(inner) => err(
                StatusCode::BAD_REQUEST,
                InternalErrorCode::QueryRejection(inner.body_text()),
            ),
            other => err(
                other.status(),
                InternalErrorCode::QueryRejection(other.body_text()),
            ),
        }
    }
}
