use axum::{
    Json,
    http::{HeaderMap, StatusCode},
};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use tracing::error;

use crate::{
    AppState,
    db::schema::user,
    routes::types::generic_internal_err::{InternalErrorCode, InternalErrorRes},
};

pub const HEADER_AUTHORIZATION: &'static str = "Authorization";

pub fn get_random_string_s() -> String {
    let mut buff = vec![0u8; 64];
    rand::fill(&mut buff);

    BASE64_URL_SAFE_NO_PAD.encode(buff)
}

// TODO(pg): Move this to a middleware
pub fn assert_auth(state: &AppState, headers: &HeaderMap) -> Result<(), StatusCode> {
    let auth_key = match headers.get(HEADER_AUTHORIZATION) {
        Some(value) => value.to_str().map_err(|_| StatusCode::BAD_REQUEST)?,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    if auth_key != state.admin_token {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(())
}

pub async fn get_user_by_username(
    username: &String,
    state: &AppState,
) -> Result<user::Model, (StatusCode, Json<InternalErrorRes>)> {
    Ok(
        match user::Entity::find_by_username(username)
            .one(&state.db)
            .await
            .map_err(|e| {
                error!("Error finding a user: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(InternalErrorRes::new(InternalErrorCode::InternalDBError)),
                )
            })? {
            Some(user) => user,
            None => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(InternalErrorRes::new(
                        InternalErrorCode::NoSuchUserFoundError,
                    )),
                ));
            }
        },
    )
}
