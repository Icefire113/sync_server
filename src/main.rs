use std::env;

use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::types::endpoints;

mod db;
mod routes;
mod util;

#[derive(Debug, Clone)]
struct AppState {
    pub db: DatabaseConnection,
    pub admin_token: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();
    info!("Starting sync_server ver: {}", env!("CARGO_PKG_VERSION"));

    let db = db::establish_connection().await;
    db::is_db_conn_ok(&db).await;

    db.get_schema_builder()
        .register(db::schema::user::Entity)
        .register(db::schema::tracked_file::Entity)
        .register(db::schema::machine_path::Entity)
        .sync(&db)
        .await
        .unwrap();
    info!("DB schema synced");

    let app_state = AppState {
        db,
        admin_token: util::get_random_string_s(),
    };
    info!("Instnace admin token: {}", app_state.admin_token);

    let app: Router = Router::new()
        .route(endpoints::VERSION, get(routes::version::version))
        .route(
            endpoints::QUERY_SYNCED_FILE,
            get(routes::api::query_discrim::query_discrim),
        )
        .route(
            endpoints::CREATE_USER,
            post(routes::api::auth::create_user::create_user),
        )
        .route(
            endpoints::GET_ALL_SYNCED_FILES,
            get(routes::api::get_all_discriminators::get_all_discriminators),
        )
        .route(
            endpoints::CREATE_SYNCED_FILE,
            post(routes::api::synced_file::create::create),
        )
        .with_state(app_state);

    let addr = format!(
        "{}:{}",
        env::var("HOST").expect("HOST should be specifed in env"),
        env::var("PORT").expect("PORT should be specifed in env")
    );
    info!("Server listening on {}", addr);

    axum::serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
