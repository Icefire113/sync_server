use api_types::{
    ApiError, ApiResponse,
    internal_err::InternalErrorCode,
    update_file::{UpdateFileReq, UpdateFileRes},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::WithRejection;
use chrono::Utc;
use entity::{
    tracked_file::{self, ActiveModel},
    user,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, TransactionTrait,
};
use tracing::error;
use uuid::Uuid;

use crate::AppState;

pub async fn update_file(
    State(state): State<AppState>,
    Extension(user): Extension<user::Model>,
    WithRejection(Path(file_id), _): WithRejection<Path<Uuid>, ApiError>,
    WithRejection(Json(req), _): WithRejection<Json<UpdateFileReq>, ApiError>,
) -> ApiResponse<Json<UpdateFileRes>> {
    let file_model = match tracked_file::Entity::find_by_id(file_id)
        .filter(tracked_file::Column::UserId.eq(user.id))
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding file for updating {:?}", e);
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

    let current_hash = file_model.hash as u64;

    let updated_model = state
        .db
        .transaction::<_, tracked_file::Model, ApiError>(|txn| {
            Box::pin(async move {
                let mut file_model: ActiveModel = file_model.into();
                // always update last updated from field
                file_model.last_updated_from = Set(req.machine_name);
                file_model.updated_at = Set(Utc::now());
                if let Some(name) = req.name {
                    file_model.name = Set(name)
                }

                if let Some(contents) = req.update_contents {
                    if contents.file_bytes.len() as u64 > state.max_file_size {
                        return Err((
                            StatusCode::PAYLOAD_TOO_LARGE,
                            InternalErrorCode::FileTooLarge,
                        )
                            .into());
                    }
                    if contents.expected_hash != current_hash {
                        return Err(
                            (StatusCode::CONFLICT, InternalErrorCode::MismatchedFileHash).into(),
                        );
                    }

                    let storage_key = format!("{}/{}", user.username, file_id);
                    state
                        .storage
                        .put(&storage_key, contents.file_bytes)
                        .await
                        .map_err(|e| {
                            error!("Error storing updated file {:?}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                InternalErrorCode::StorageBackendError,
                            )
                        })?;

                    file_model.hash = Set(contents.hash as i64);
                }

                Ok(file_model.update(txn).await.map_err(|e| {
                    error!("Error updating file {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        InternalErrorCode::InternalDBError,
                    )
                })?)
            })
        })
        .await.map_err(|e| {
            match e {
                sea_orm::TransactionError::Connection(db_err) => {
                    error!("Error updating file, you may want to check for corrupted/ partially updated data {:?}", db_err);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        InternalErrorCode::InternalDBError,
                    ).into()
                },
                sea_orm::TransactionError::Transaction(e) => {
                    e
                },
            }
        })?;

    Ok((
        StatusCode::CREATED,
        Json(UpdateFileRes {
            name: updated_model.name,
            hash: updated_model.hash as u64,
            machine_name: updated_model.last_updated_from,
            updated_at: updated_model.updated_at,
        }),
    ))
}
