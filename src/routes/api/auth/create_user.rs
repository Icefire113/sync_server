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
        generic_internal_err::{InternalErrorCodes, InternalErrorRes},
    },
    util::get_random_string_s,
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUserReq>,
) -> Result<(StatusCode, Json<CreateUserRes>), (StatusCode, Json<InternalErrorRes>)> {
    // TODO: Find a better way to lock down account creation
    // util::assert_auth(&state, &headers)?;

    let pw = get_random_string_s();
    let salt = SaltString::generate(&mut OsRng);
    let hashed = Argon2::default()
        .hash_password(pw.as_bytes(), &salt)
        .map_err(|e| {
            error!("Error hashing password: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(InternalErrorCodes::PasswordHashError)),
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
            Json(InternalErrorRes::new(InternalErrorCodes::InternalDBError)),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(CreateUserRes {
            username: input.username,
            access_key: pw,
        }),
    ))
}
