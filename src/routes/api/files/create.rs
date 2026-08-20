use api_types::{
    ApiError, ApiResponse,
    create_file::{CreateFileReq, CreateFileRes},
    internal_err::InternalErrorCode,
};
use axum::{Extension, Json, extract::State, http::StatusCode};
use axum_extra::extract::WithRejection;
use chrono::Utc;
use entity::{tracked_file, user};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use tracing::error;
use uuid::Uuid;

use crate::AppState;

pub async fn create_file(
    State(state): State<AppState>,
    Extension(user): Extension<user::Model>,
    WithRejection(Json(req), _): WithRejection<Json<CreateFileReq>, ApiError>,
) -> ApiResponse<Json<CreateFileRes>> {
    let file_id = Uuid::new_v4();
    let storage_key = format!("{}/{}", user.username, file_id);

    if req.file_bytes.len() as u64 > state.max_file_size {
        return Err((
            StatusCode::PAYLOAD_TOO_LARGE,
            InternalErrorCode::FileTooLarge,
        )
            .into());
    }

    state
        .storage
        .put(&storage_key, req.file_bytes)
        .await
        .map_err(|e| {
            error!("Error storing file {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::StorageBackendError,
            )
        })?;
    let tracked_file = match (tracked_file::ActiveModel {
        id: Set(file_id),
        name: Set(req.name),
        hash: Set(req.hash as i64),
        last_updated_from: Set(req.machine_name),
        updated_at: Set(Utc::now()),
        user_id: Set(user.id),
        ..Default::default()
    })
    .insert(&state.db)
    .await
    {
        Ok(model) => model,
        Err(e) => {
            error!("Error creating new tracked file {:?}", e);
            if let Err(cleanup_err) = state.storage.delete(&storage_key).await {
                error!(
                    "Failed to clean up orphan blob after DB insert failure: {:?}",
                    cleanup_err
                );
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::InternalDBError,
            )
                .into());
        }
    };

    Ok((
        StatusCode::CREATED,
        Json(CreateFileRes {
            id: tracked_file.id,
        }),
    ))
}
