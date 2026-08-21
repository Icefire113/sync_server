use axum::{
    Extension,
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
};
use sha2::{Digest, Sha256};
use tracing::error;

use api_types::{ApiError, internal_err::InternalErrorCode};
use entity::{access_token, user};

use crate::AppState;

const AUTH_HEADER: &str = "Authorization";
pub const ACCESS_TOKEN_PREFIX: &str = "cfs_";

/// Asserts that the user attached to the request is enabled, note that this required that this middleware is run after `check_authenticated` as that is where the user is attached
pub async fn check_enabled(
    Extension(user): Extension<user::Model>,
    req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    if !user.enabled {
        Err((
            StatusCode::UNAUTHORIZED,
            InternalErrorCode::AccountNotEnabled,
        )
            .into())
    } else {
        Ok(next.run(req).await)
    }
}

pub async fn check_user_role_atleast(
    Extension(user): Extension<user::Model>,
    Extension(required_role): Extension<user::Role>,
    req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    if user.role >= required_role {
        Ok(next.run(req).await)
    } else {
        Err((StatusCode::FORBIDDEN, InternalErrorCode::InsufficientRole).into())
    }
}

pub async fn check_authenticated(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<Response, ApiError> {
    match get_token(&headers) {
        Some(token) => {
            let token_hash: Vec<u8> = Sha256::digest(token.as_bytes()).to_vec();

            match access_token::Entity::find()
                .filter(access_token::Column::TokenHash.eq(token_hash))
                .one(&state.db)
                .await
                .map_err(|e| {
                    error!("Error finding access token for auth {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        InternalErrorCode::InternalDBError,
                    )
                })? {
                Some(tok) => {
                    // is the token expired?
                    if tok.expires_at <= Utc::now() {
                        return Err(
                            (StatusCode::UNAUTHORIZED, InternalErrorCode::TokenExpired).into()
                        );
                    }
                    // is the token revoked?
                    if let Some(exp_time) = tok.revoked_at
                        && exp_time <= Utc::now()
                    {
                        return Err(
                            (StatusCode::UNAUTHORIZED, InternalErrorCode::TokenRevoked).into()
                        );
                    }
                    // grab the user that the token is for
                    let user: user::Model = tok
                        .find_related(user::Entity)
                        .one(&state.db)
                        .await
                        .map_err(|e| {
                            error!("Error finding access token for auth {:?}", e);
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                InternalErrorCode::InternalDBError,
                            )
                        })?
                        .ok_or_else(|| {
                            error!("Failed to find a user that we have a token for, user_id: {} token id: {}", tok.user_id, tok.id);
                            (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            InternalErrorCode::InternalError,
                        )})?;
                    let mut tok: access_token::ActiveModel = tok.into();
                    tok.last_used_at = Set(Some(Utc::now()));
                    tok.save(&state.db).await.map_err(|e| {
                        error!("Error updating access token for auth {:?}", e);
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            InternalErrorCode::InternalDBError,
                        )
                    })?;
                    req.extensions_mut().insert(user);
                    Ok(next.run(req).await)
                }
                None => Err((StatusCode::UNAUTHORIZED, InternalErrorCode::Unauthorized).into()),
            }
        }
        None => Err((StatusCode::UNAUTHORIZED, InternalErrorCode::Unauthorized).into()),
    }
}

/// Gets the auth token from the headers, returns only the hex part, stripping off the `Bearer cfs_` prefix
fn get_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(AUTH_HEADER)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")?
        .strip_prefix(ACCESS_TOKEN_PREFIX)
}
