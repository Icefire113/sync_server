use axum::Router;

use crate::AppState;

pub mod auth;

pub fn build_api_router(state: AppState) -> Router<AppState> {
    let api_router = Router::new().nest("/auth", auth::build_auth_router(state));

    api_router
}
