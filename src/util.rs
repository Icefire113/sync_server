use axum::http::{HeaderMap, StatusCode};
use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use rand::{TryRngCore, rngs::OsRng};

use crate::AppState;

pub const HEADER_AUTHORIZATION: &str = "Authorization";

pub fn get_random_string_s() -> String {
    let mut buff = vec![0u8; 64];
    OsRng
        .try_fill_bytes(&mut buff)
        .expect("Failed to get 64 bytes of random data for the instance admin token");
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
