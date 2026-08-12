use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use entity::user;
use sea_orm::EntityTrait;
use tracing::error;

use crate::{
    AppState,
    routes::types::{ApiResponse, get_user::GetUserRes, internal_err::InternalErrorCode},
};

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    Path(target_user_id): Path<i64>,
) -> ApiResponse<Json<GetUserRes>> {
    let user = match user::Entity::find_by_id(target_user_id)
        .one(&state.db)
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(InternalErrorCode::NoSuchUserFound.into()),
                ));
            }
        },
        Err(e) => {
            error!("Error finding user by id {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorCode::InternalDBError.into()),
            ));
        }
    };

    Ok((
        StatusCode::OK,
        Json(GetUserRes {
            id: user.id,
            username: user.username,
            role: user.role,
            created_at: user.created_at,
            enabled: user.enabled,
        }),
    ))
}
