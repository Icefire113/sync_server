use api_types::{
    ApiError, ApiResponse,
    get_file::{GetFileContentRes, GetFileInfoRes},
    internal_err::InternalErrorCode,
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::WithRejection;
use entity::{tracked_file, user};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::{error, warn};
use uuid::Uuid;

use crate::{AppState, storage::StorageError};

pub async fn get_file_info(
    State(state): State<AppState>,
    Extension(user): Extension<user::Model>,
    WithRejection(Path(file_id), _): WithRejection<Path<Uuid>, ApiError>,
) -> ApiResponse<Json<GetFileInfoRes>> {
    let file_model = match tracked_file::Entity::find_by_id(file_id)
        .filter(tracked_file::Column::UserId.eq(user.id))
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Database error looking up file {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::InternalDBError,
            )
        })? {
        Some(model) => model,
        None => return Err((StatusCode::NOT_FOUND, InternalErrorCode::FileNotFound).into()),
    };
    Ok((
        StatusCode::OK,
        Json(GetFileInfoRes {
            id: file_model.id,
            name: file_model.name,
            hash: file_model.hash as u64,
            last_updated_from: file_model.last_updated_from,
            updated_at: file_model.updated_at,
            deleted_at: file_model.deleted_at,
        }),
    ))
}

pub async fn get_file_contents(
    State(state): State<AppState>,
    Extension(user): Extension<user::Model>,
    WithRejection(Path(file_id), _): WithRejection<Path<Uuid>, ApiError>,
) -> ApiResponse<GetFileContentRes> {
    // if we fail to find a matching file id that belongs to the user that is making the request, treat it as if it does not exist
    // although its HIGHLY unlikely that someone manages to collide a uuid
    let file_model = match tracked_file::Entity::find_by_id(file_id)
        .filter(tracked_file::Column::UserId.eq(user.id))
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Database error looking up file {:?}", e);
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

    let storage_key = format!("{}/{}", user.username, file_model.id);

    let bytes = state.storage.get(&storage_key).await.map_err(|e| match e {
        StorageError::Internal(_) => {
            error!("Error fetching data from storage {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::StorageBackendError,
            )
        }
        StorageError::NotFound => {
            warn!("We have a db record for {} but nothing in storage", {
                storage_key
            });
            (StatusCode::NOT_FOUND, InternalErrorCode::FileNotInStorage)
        }
    })?;

    Ok((StatusCode::OK, bytes))
}
