use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, EntityTrait};
use tracing::error;

use entity::user;

use crate::{
    AppState,
    routes::types::{ApiResponse, internal_err::InternalErrorCode},
};

pub async fn delete_user(
    State(state): State<AppState>,
    Path(target_user_id): Path<i64>,
) -> ApiResponse<()> {
    let user_model: user::ActiveModel = match user::Entity::find_by_id(target_user_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding user by id to delete {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::InternalDBError.into(),
            )
        })? {
        Some(m) => m,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                InternalErrorCode::NoSuchUserFound.into(),
            ));
        }
    }
    .into();

    user_model.delete(&state.db).await.map_err(|e| {
        error!("Error deleting user {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            InternalErrorCode::InternalDBError.into(),
        )
    })?;

    Ok((StatusCode::NO_CONTENT, ()))
}
