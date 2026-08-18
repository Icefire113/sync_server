use api_types::{ApiError, ApiResponse, get_file::GetFileInfoRes, internal_err::InternalErrorCode};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::WithRejection;
use entity::{tracked_file, user};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::error;
use uuid::Uuid;

use crate::AppState;

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
