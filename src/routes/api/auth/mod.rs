use axum::{
    Router, middleware,
    routing::{get, post},
};

use crate::{
    AppState,
    middleware::auth::{check_authenticated, check_enabled},
    routes::api::auth::{
        create_token::create_token, create_user::create_user, token_info::token_info,
        request_token::request_token, revoke_token::revoke_token,
    },
};

mod create_token;
mod create_user;
mod token_info;
mod request_token;
mod revoke_token;

pub fn build_auth_router(state: AppState) -> Router<AppState> {
    let auth_router: Router<AppState> = Router::new()
        .route("/create_user", post(create_user))
        .route(
            "/token_info",
            get(token_info)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        )
        .route(
            "/revoke_token",
            post(revoke_token)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        )
        .route("/request_token", post(request_token))
        .route(
            "/create_token",
            post(create_token)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        );

    auth_router
}
