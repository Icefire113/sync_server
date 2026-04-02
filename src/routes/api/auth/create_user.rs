use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr};
use tracing::error;

use crate::{
    AppState,
    db::schema::user,
    routes::types::{
        create_user::{CreateUserReq, CreateUserRes},
        generic_internal_err::{InternalErrorCode, InternalErrorRes},
    },
    util::get_random_string_s,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUserReq>,
) -> Result<(StatusCode, Json<CreateUserRes>), (StatusCode, Json<InternalErrorRes>)> {
    // TODO: Find a better way to lock down account creation
    // util::assert_auth(&state, &headers)?;
    if input.username.len() < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(InternalErrorRes::new(InternalErrorCode::UsernameTooShort)),
        ));
    }

    let access_key = get_random_string_s();
    let hashed = Argon2::default()
        .hash_password(access_key.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|e| {
            error!("Error hashing access key: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(
                    InternalErrorCode::AccessKeyHashError,
                )),
            )
        })?
        .to_string();

    let user = user::ActiveModel {
        username: Set(input.username.to_owned()),
        access_key: Set(hashed.to_owned()),
        ..Default::default()
    };

    // TODO: Test if username is already taken and return a custom error if so
    user.insert(&state.db).await.map_err(|e: DbErr| {
        error!("Error creating user: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(InternalErrorRes::new(InternalErrorCode::InternalDBError)),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(CreateUserRes {
            username: input.username,
            access_key,
        }),
    ))
}
