use axum::{Extension, Json, extract::State, http::StatusCode};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
};
use tracing::error;

use crate::{
    AppState,
    db::schema::{access_token, user},
    routes::types::{
        ApiResponse,
        internal_err::{InternalErrorCode, InternalErrorRes},
        revoke_token::RevokeTokenReq,
    },
};

pub async fn revoke_token(
    State(state): State<AppState>,
    Extension(user): Extension<user::Model>,
    Json(req): Json<RevokeTokenReq>,
) -> ApiResponse<()> {
    let token_model: access_token::Model = match access_token::Entity::find_by_id(req.id)
        .filter(access_token::Column::UserId.eq(user.id))
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding token by id {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(InternalErrorCode::InternalDBError)),
            )
        })? {
        Some(token) => token,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(InternalErrorRes::new(InternalErrorCode::TokenNotFound)),
            ));
        }
    };

    if token_model.revoked_at.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(InternalErrorRes::new(
                InternalErrorCode::TokenAlreadyRevoked,
            )),
        ));
    }

    let tokens = user
        .find_related(access_token::Entity)
        .all(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding token by id {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(InternalErrorCode::InternalDBError)),
            )
        })?;

    let num_non_revoked_tokens =
        tokens.iter().fold(
            0,
            |acc, t| if t.revoked_at.is_none() { acc + 1 } else { acc },
        );

    // dont allow user to revoke last token
    if num_non_revoked_tokens == 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(InternalErrorRes::new(
                InternalErrorCode::CannotRevokeAllTokens,
            )),
        ));
    }

    let mut token_model: access_token::ActiveModel = token_model.into();
    token_model.revoked_at = Set(Some(Utc::now()));
    token_model.update(&state.db).await.map_err(|e| {
        error!("Error finding token by id {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(InternalErrorRes::new(InternalErrorCode::InternalDBError)),
        )
    })?;

    Ok((StatusCode::NO_CONTENT, ()))
}
