use axum::{
    Router, middleware,
    routing::{delete, get, patch, post},
};

use crate::{
    AppState,
    middleware::auth::{check_authenticated, check_enabled},
};

mod create;
mod delete;
mod get;
mod update;

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
            "/{id}/info",
            get(get::get_file_info)
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
        )
        .route(
            "/{id}",
            delete(delete::delete_file)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        )
        .route(
            "/{id}",
            patch(update::update_file)
                .route_layer(middleware::from_fn(check_enabled))
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    check_authenticated,
                )),
        );

    files_router
}
