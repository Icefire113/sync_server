use axum::Router;

use crate::AppState;

pub fn build_files_router(state: AppState) -> Router<AppState> {
    let files_router = Router::new();

    files_router
}
