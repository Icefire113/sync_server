use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::WithRejection;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, TransactionTrait};
use sha2::{Digest, Sha256};
use tracing::error;

use entity::{
    access_token,
    user::{self, Model},
};

use crate::{
    AppState,
    middleware::auth::ACCESS_TOKEN_PREFIX,
    routes::types::{
        ApiError, ApiResponse,
        create_user::{CreateUserReq, CreateUserRes},
        internal_err::InternalErrorCode,
    },
    util::get_random_string_s,
};

pub async fn create_user(
    State(state): State<AppState>,
    WithRejection(Json(input), _): WithRejection<Json<CreateUserReq>, ApiError>,
) -> ApiResponse<Json<CreateUserRes>> {
    if input.username.is_empty() {
        return Err((StatusCode::BAD_REQUEST, InternalErrorCode::UsernameTooShort).into());
    } else if input.username.len() > 50 {
        return Err((StatusCode::BAD_REQUEST, InternalErrorCode::UsernameTooLong).into());
    } else if !input
        .username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err((
            StatusCode::BAD_REQUEST,
            InternalErrorCode::UsernameContainsInvalidChars,
        )
            .into());
    }

    // Check if username is taken
    match user::Entity::find_by_username(&input.username)
        .one(&state.db)
        .await
    {
        Ok(user) => match user {
            Some(_) => Err((StatusCode::BAD_REQUEST, InternalErrorCode::UsernameTaken).into()),
            None => {
                let hashed = Argon2::default()
                    .hash_password(input.password.as_bytes(), &SaltString::generate(&mut OsRng))
                    .map_err(|e| {
                        error!("Error hashing access key: {:?}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            InternalErrorCode::PasswordHash,
                        )
                    })?
                    .to_string();

                let username = input.username.clone();
                let access_token = get_random_string_s();
                let token_hash = Sha256::digest(&access_token).to_vec();
                let user = state
                    .db
                    .transaction::<_, Model, DbErr>(|txn| {
                        Box::pin(async move {
                            let user = user::ActiveModel {
                                username: Set(username),
                                password_hash: Set(hashed),
                                ..Default::default()
                            }
                            .insert(txn)
                            .await?;

                            access_token::ActiveModel {
                                expires_at: Set(Utc::now() + chrono::Duration::days(365)),
                                token_hash: Set(token_hash),
                                user_id: Set(user.id),
                                ..Default::default()
                            }
                            .insert(txn)
                            .await?;

                            Ok(user)
                        })
                    })
                    .await
                    .map_err(|e| {
                        error!("Error creating user: {:?}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            InternalErrorCode::InternalDBError,
                        )
                    })?;

                Ok((
                    StatusCode::CREATED,
                    Json(CreateUserRes {
                        id: user.id,
                        username: input.username,
                        access_token: format!("{}{}", ACCESS_TOKEN_PREFIX, access_token),
                    }),
                ))
            }
        },
        Err(e) => {
            error!("Error finding user: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::InternalDBError,
            )
                .into())
        }
    }
}
