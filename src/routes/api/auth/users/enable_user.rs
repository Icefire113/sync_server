use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use tracing::error;

use crate::{
    AppState,
    db::schema::user,
    routes::types::{ApiResponse, internal_err::InternalErrorCode},
};

pub async fn enable_user(
    State(state): State<AppState>,
    Path(target_user_id): Path<i64>,
) -> ApiResponse<()> {
    let user_model: user::Model = match user::Entity::find_by_id(target_user_id)
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding user by id to enable {:?}", e);
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

    if user_model.enabled {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(InternalErrorCode::UserAlreadyEnabled.into()),
        ));
    }

    let mut user_model: user::ActiveModel = user_model.into();
    user_model.enabled = Set(true);

    user_model.save(&state.db).await.map_err(|e| {
        error!("Error saving enabled user {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(InternalErrorCode::InternalDBError.into()),
        )
    })?;

    Ok((StatusCode::NO_CONTENT, ()))
}
