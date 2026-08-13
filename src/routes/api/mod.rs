use axum::Router;

use crate::AppState;

mod auth;
mod files;

pub fn build_api_router(state: AppState) -> Router<AppState> {
    let api_router = Router::new()
        .nest("/auth", auth::build_auth_router(state.clone()))
        .nest("/files", files::build_files_router(state.clone()));

    api_router
}
