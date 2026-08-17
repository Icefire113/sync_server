use axum::{Extension, Json, extract::State, http::StatusCode};
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
        create_token::{CreateTokenReq, CreateTokenRes},
        internal_err::InternalErrorCode,
    },
    util::get_random_string_s,
};

pub async fn create_token(
    State(state): State<AppState>,
    Extension(user): Extension<user::Model>,
    Json(req): Json<CreateTokenReq>,
) -> ApiResponse<Json<CreateTokenRes>> {
    let token = get_random_string_s();

    let token_model = access_token::ActiveModel {
        name: Set(req.name),
        expires_at: Set(Utc::now() + Duration::days(req.duration_days.into())),
        user_id: Set(user.id),
        token_hash: Set(Sha256::digest(token.as_bytes()).to_vec()),
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
        Json(CreateTokenRes {
            token: format!("{}{}", ACCESS_TOKEN_PREFIX, token),
            // if this panics, then the token_model response from the db was an error, but somehow our map_err try didnt catch it
            token_id: token_model.id.unwrap(),
        }),
    ))
}
