use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use sha2::{Digest, Sha256};
use tracing::error;

use entity::{access_token, user};

use crate::{
    AppState,
    middleware::auth::ACCESS_TOKEN_PREFIX,
    routes::types::{
        ApiResponse,
        internal_err::InternalErrorCode,
        request_token::{RequestTokenReq, RequestTokenRes},
    },
    util::get_random_string_s,
};

pub async fn request_token(
    State(state): State<AppState>,
    Json(req): Json<RequestTokenReq>,
) -> ApiResponse<Json<RequestTokenRes>> {
    let user_model = match user::Entity::find_by_username(req.username)
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding user {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::InternalDBError.into(),
            )
        })? {
        Some(user) => user,
        None => {
            // dummy to prevent timing attacks
            let _ = Argon2::default()
                .hash_password(req.password.as_bytes(), &SaltString::generate(&mut OsRng));
            return Err((
                StatusCode::BAD_REQUEST,
                InternalErrorCode::InvalidUsernameOrPassword.into(),
            ));
        }
    };
    Argon2::default()
        .verify_password(
            req.password.as_bytes(),
            &PasswordHash::new(&user_model.password_hash).map_err(|e| {
                error!(
                    "Failed to constrct PasswordHash from password hash saved in DB, user_id: {}, {}",
                    user_model.id, e
                );
                (
                    StatusCode::BAD_REQUEST,
                    InternalErrorCode::InvalidUsernameOrPassword.into(),
                )
            })?,
        )
        .map_err(|e| match e {
            argon2::password_hash::Error::Password => (
                StatusCode::BAD_REQUEST,
                InternalErrorCode::InvalidUsernameOrPassword.into(),
            ),
            e => {
                error!("Error verifying password {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    InternalErrorCode::HashPasswordVerify.into(),
                )
            }
        })?;

    let access_token: String = get_random_string_s();

    let token_model = access_token::ActiveModel {
        name: Set(req.token_name),
        expires_at: Set(Utc::now() + Duration::days(req.duration_days.into())),
        user_id: Set(user_model.id),
        token_hash: Set(Sha256::digest(access_token.as_bytes()).to_vec()),
        ..Default::default()
    }
    .save(&state.db)
    .await
    .map_err(|e| {
        error!("Error saving new token {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            InternalErrorCode::InternalDBError.into(),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(RequestTokenRes {
            token: format!("{}{}", ACCESS_TOKEN_PREFIX, access_token),
            // if this panics, then the token_model response from the db was an error, but somehow our map_err try didnt catch it
            token_id: token_model.id.unwrap(),
        }),
    ))
}
