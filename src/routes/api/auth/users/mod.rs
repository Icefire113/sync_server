use axum::{
    Extension, Router, middleware,
    routing::{patch, post},
};

use crate::{
    AppState,
    db::schema::user::Role,
    middleware::auth::{check_authenticated, check_enabled, check_user_role_atleast},
    routes::api::auth::users::{create_user::create_user, enable_user::enable_user},
};

mod create_user;
mod enable_user;

pub fn build_users_router(state: AppState) -> Router<AppState> {
    let users_router: Router<AppState> = Router::new().route("/create", post(create_user)).route(
        "/enable/{id}",
        patch(enable_user)
            .route_layer(middleware::from_fn(check_user_role_atleast))
            .layer(Extension(Role::Admin))
            .route_layer(middleware::from_fn(check_enabled))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                check_authenticated,
            )),
    );

    users_router
}
