use axum::{Router, routing::post};

use crate::{AppState, routes::api::auth::users::create_user::create_user};

mod create_user;

pub fn build_users_router(state: AppState) -> Router<AppState> {
    let users_router: Router<AppState> = Router::new().route("/create", post(create_user));

    users_router
}
