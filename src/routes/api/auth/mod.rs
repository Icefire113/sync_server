use axum::Router;

use crate::AppState;

mod tokens;
mod users;

pub fn build_auth_router(state: AppState) -> Router<AppState> {
    let auth_router: Router<AppState> = Router::new()
        .nest("/tokens", tokens::build_tokens_router(state.clone()))
        .nest("/users", users::build_users_router(state.clone()));

    auth_router
}
