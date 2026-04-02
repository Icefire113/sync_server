use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DbErr};
use tracing::error;

use crate::{
    AppState,
    db::schema::user,
    routes::api::types::{
        create_user::{CreateUserReq, CreateUserRes},
        generic_internal_err::{InternalErrorCodes, InternalErrorRes},
    },
};

pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUserReq>,
) -> Result<(StatusCode, Json<CreateUserRes>), (StatusCode, Json<InternalErrorRes>)> {
    // TODO: Find a better way to lock down account creation
    // util::assert_auth(&state, &headers)?;

    let user = user::ActiveModel {
        username: Set(input.username.to_owned()),
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
        }),
    ))
}
