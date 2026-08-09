use axum::{
    Router, middleware,
    routing::{get, patch, post},
};

use crate::{
    AppState,
    middleware::auth::{check_authenticated, check_enabled},
    routes::api::auth::tokens::{
        create::create_token, info::token_info, request::request_token, revoke::revoke_token,
    },
};

mod create;
mod info;
mod request;
mod revoke;

pub fn build_tokens_router(state: AppState) -> Router<AppState> {
    let tokens_router: Router<AppState> = Router::new()
        .route(
            "/info",
            get(token_info)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        )
        .route(
            "/revoke",
            patch(revoke_token)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        )
        .route("/request", post(request_token))
        .route(
            "/create",
            post(create_token)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        );

    tokens_router
}
