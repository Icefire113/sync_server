use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::WithRejection;
use sea_orm::{ActiveModelTrait, EntityTrait};
use tracing::error;

use api_types::{ApiError, ApiResponse, internal_err::InternalErrorCode};
use entity::user;

use crate::AppState;

pub async fn delete_user(
    State(state): State<AppState>,
    WithRejection(Path(target_user_id), _): WithRejection<Path<i64>, ApiError>,
) -> ApiResponse<()> {
    let user_model: user::ActiveModel = match user::Entity::find_by_id(target_user_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding user by id to delete {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::InternalDBError,
            )
        })? {
        Some(m) => m,
        None => {
            return Err((StatusCode::NOT_FOUND, InternalErrorCode::NoSuchUserFound).into());
        }
    }
    .into();

    user_model.delete(&state.db).await.map_err(|e| {
        error!("Error deleting user {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            InternalErrorCode::InternalDBError,
        )
    })?;

    Ok((StatusCode::NO_CONTENT, ()))
}
