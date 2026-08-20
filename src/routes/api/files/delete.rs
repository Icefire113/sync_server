use api_types::{ApiError, ApiResponse, internal_err::InternalErrorCode};
use axum::{
    Extension,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::WithRejection;
use chrono::Utc;
use entity::{
    tracked_file::{self},
    user,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use tracing::error;
use uuid::Uuid;

use crate::AppState;

pub async fn delete_file(
    State(state): State<AppState>,
    Extension(user): Extension<user::Model>,
    WithRejection(Path(file_id), _): WithRejection<Path<Uuid>, ApiError>,
) -> ApiResponse<()> {
    let file_model = match tracked_file::Entity::find_by_id(file_id)
        .filter(tracked_file::Column::UserId.eq(user.id))
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding file for deleting {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::InternalDBError,
            )
        })? {
        Some(model) => model,
        None => return Err((StatusCode::NOT_FOUND, InternalErrorCode::FileNotFound).into()),
    };
    if file_model.deleted_at.is_some() {
        return Err((StatusCode::NOT_FOUND, InternalErrorCode::FileDeleted).into());
    }

    // delete from storage first
    let storage_key = format!("{}/{}", user.username, file_id);
    state.storage.delete(&storage_key).await.map_err(|e| {
        error!("Storage error deleting file {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            InternalErrorCode::StorageBackendError,
        )
    })?;

    let mut file_model: tracked_file::ActiveModel = file_model.into();
    file_model.deleted_at = Set(Some(Utc::now()));
    file_model.update(&state.db).await.map_err(|e| {
        error!("Error marking file as deleted {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            InternalErrorCode::InternalDBError,
        )
    })?;

    Ok((StatusCode::NO_CONTENT, ()))
}
