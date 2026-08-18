use axum::{
    Extension, Json,
    extract::{Query, State},
    http::StatusCode,
};
use axum_extra::extract::WithRejection;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use sha2::{Digest, Sha256};
use tracing::error;

use entity::{access_token, user};

use crate::{
    AppState,
    middleware::auth::ACCESS_TOKEN_PREFIX,
    routes::types::{
        ApiError, ApiResponse,
        get_token_info::{GetTokenInfoReq, GetTokenInfoRes},
        internal_err::InternalErrorCode,
    },
};

#[axum::debug_handler]
pub async fn token_info(
    State(state): State<AppState>,
    Extension(user): Extension<user::Model>,
    WithRejection(Query(req_params), _): WithRejection<Query<GetTokenInfoReq>, ApiError>,
) -> ApiResponse<Json<GetTokenInfoRes>> {
    let token: &str = req_params.token.strip_prefix(ACCESS_TOKEN_PREFIX).ok_or((
        StatusCode::BAD_REQUEST,
        InternalErrorCode::InvalidTokenPrefix,
    ))?;
    let token_hash = Sha256::digest(token.as_bytes()).to_vec();

    let token_model = match access_token::Entity::find()
        .filter(access_token::Column::TokenHash.eq(token_hash))
        .filter(access_token::Column::UserId.eq(user.id))
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Database error finding access token {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                InternalErrorCode::InternalDBError,
            )
        })? {
        Some(token) => token,
        None => {
            return Err((StatusCode::NOT_FOUND, InternalErrorCode::TokenNotFound).into());
        }
    };

    Ok((
        StatusCode::OK,
        Json(GetTokenInfoRes {
            id: token_model.id,
            name: token_model.name,
            created_at: token_model.created_at,
            expires_at: token_model.expires_at,
            revoked_at: token_model.revoked_at,
            last_used: token_model.last_used_at,
        }),
    ))
}
