use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr, TransactionTrait};
use sha2::{Digest, Sha256};
use tracing::error;

use crate::{
    AppState,
    db::schema::{access_token, user},
    middleware::auth::ACCESS_TOKEN_PREFIX,
    routes::types::{
        ApiResponse,
        create_user::{CreateUserReq, CreateUserRes},
        internal_err::InternalErrorCode,
    },
    util::get_random_string_s,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUserReq>,
) -> ApiResponse<Json<CreateUserRes>> {
    if input.username.len() < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(InternalErrorCode::UsernameTooShort.into()),
        ));
    } else if input.username.len() > 50 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(InternalErrorCode::UsernameTooLong.into()),
        ));
    }

    // Check if username is taken
    match user::Entity::find_by_username(&input.username)
        .one(&state.db)
        .await
    {
        Ok(user) => match user {
            Some(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(InternalErrorCode::UsernameTaken.into()),
                ));
            }
            None => {
                let hashed = Argon2::default()
                    .hash_password(input.password.as_bytes(), &SaltString::generate(&mut OsRng))
                    .map_err(|e| {
                        error!("Error hashing access key: {:?}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(InternalErrorCode::PasswordHash.into()),
                        )
                    })?
                    .to_string();

                let username = input.username.clone();
                let access_token = get_random_string_s();
                let token_hash = Sha256::digest(&access_token).to_vec();
                state
                    .db
                    .transaction::<_, (), DbErr>(|txn| {
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

                            Ok(())
                        })
                    })
                    .await
                    .map_err(|e| {
                        error!("Error creating user: {:?}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(InternalErrorCode::InternalDBError.into()),
                        )
                    })?;

                Ok((
                    StatusCode::CREATED,
                    Json(CreateUserRes {
                        username: input.username,
                        access_token: format!("{}{}", ACCESS_TOKEN_PREFIX, access_token),
                    }),
                ))
            }
        },
        Err(e) => {
            error!("Error finding user: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorCode::InternalDBError.into()),
            ));
        }
    }
}
