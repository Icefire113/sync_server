use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, TryIntoModel};
use tracing::error;

use entity::user;

use crate::{
    AppState,
    routes::types::{
        ApiResponse,
        internal_err::InternalErrorCode,
        update_user::{UpdateUserReq, UpdateUserRes},
    },
};

pub async fn update_user(
    State(state): State<AppState>,
    Path(target_user_id): Path<i64>,
    Json(req): Json<UpdateUserReq>,
) -> ApiResponse<Json<UpdateUserRes>> {
    let user_model: user::Model = match user::Entity::find_by_id(target_user_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding user by id to update {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorCode::InternalDBError.into()),
            )
        })? {
        Some(m) => m,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(InternalErrorCode::NoSuchUserFound.into()),
            ));
        }
    };

    let mut user_model: user::ActiveModel = user_model.into();
    if let Some(v) = req.enabled {
        user_model.enabled = Set(v);
    }
    if let Some(role) = req.role {
        user_model.role = Set(role);
    }

    let user_model: user::Model = user_model
        .save(&state.db)
        .await
        .map_err(|e| {
            error!("Error saving updating user {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorCode::InternalDBError.into()),
            )
        })?
        .try_into_model()
        .map_err(|e| {
            error!(
                "Error turning user active model into model, this is a bug, {:?}",
                e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorCode::InternalDBError.into()),
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(UpdateUserRes {
            id: user_model.id,
            role: user_model.role,
            username: user_model.username,
            enabled: user_model.enabled,
            created_at: user_model.created_at,
        }),
    ))
}
