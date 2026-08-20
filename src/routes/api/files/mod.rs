use axum::{
    Router, middleware,
    routing::{get, head, post},
};

use crate::{
    AppState,
    middleware::auth::{check_authenticated, check_enabled},
};

mod create;
mod get;

pub fn build_files_router(state: AppState) -> Router<AppState> {
    let files_router = Router::new()
        .route(
            "/",
            post(create::create_file)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        )
        .route(
            "/{id}",
            head(get::get_file_info)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        )
        .route(
            "/{id}",
            get(get::get_file_contents)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        );

    files_router
}
