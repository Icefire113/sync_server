use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr};
use tracing::error;

use crate::{
    AppState,
    db::schema::user,
    routes::api::types::create_user::{CreateUserReq, CreateUserRes},
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUserReq>,
) -> Result<(StatusCode, Json<CreateUserRes>), StatusCode> {
    // TODO: Find a better way to lock down account creation
    // util::assert_auth(&state, &headers)?;

    let user = user::ActiveModel {
        username: Set(input.username.clone()),
        ..Default::default()
    };
    // TODO: Test if username is already taken and return a custom error if so
    user.insert(&state.db).await.map_err(|e: DbErr| {
        error!("Error creating user: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((
        StatusCode::CREATED,
        Json(CreateUserRes {
            username: Some(input.username),
            errors: None,
        }),
    ))
}
