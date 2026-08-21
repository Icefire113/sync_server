use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::WithRejection;
use sea_orm::EntityTrait;
use tracing::error;

use api_types::{ApiError, ApiResponse, get_user::GetUserRes, internal_err::InternalErrorCode};
use entity::user;

use crate::AppState;

#[axum::debug_handler]
pub async fn get_user(
    State(state): State<AppState>,
    WithRejection(Path(target_user_id), _): WithRejection<Path<i64>, ApiError>,
) -> ApiResponse<Json<GetUserRes>> {
    let user = match user::Entity::find_by_id(target_user_id)
        .one(&state.db)
        .await
    {
        Ok(user) => match user {
            Some(user) => user,
            None => {
                return Err((StatusCode::NOT_FOUND, InternalErrorCode::NoSuchUserFound).into());
            }
        },
        Err(e) => {
            error!("Error finding user by id {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::InternalDBError,
            )
                .into());
        }
    };

    Ok((
        StatusCode::OK,
        Json(GetUserRes {
            id: user.id,
            username: user.username,
            role: user.role.into(),
            created_at: user.created_at,
            enabled: user.enabled,
        }),
    ))
}
