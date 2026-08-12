use axum::{
    Extension, Router, middleware,
    routing::{delete, patch, post},
};

use crate::{
    AppState,
    db::schema::user::Role,
    middleware::auth::{check_authenticated, check_enabled, check_user_role_atleast},
    routes::api::auth::users::{
        create_user::create_user, delete_user::delete_user, update_user::update_user,
    },
};

mod create_user;
mod delete_user;
mod update_user;

pub fn build_users_router(state: AppState) -> Router<AppState> {
    let users_router: Router<AppState> = Router::new()
        .route("/create", post(create_user))
        .route(
            "/{id}",
            delete(delete_user)
                .route_layer(middleware::from_fn(check_user_role_atleast))
                .layer(Extension(Role::Admin))
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        )
        .route(
            "/{id}",
            patch(update_user)
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
