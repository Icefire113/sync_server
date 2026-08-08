use std::env;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::routes::api::{self};

mod db;
mod middleware;
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
    dotenv().ok();
    info!("Starting sync_server ver: {}", env!("CARGO_PKG_VERSION"));

    let db: DatabaseConnection = db::establish_connection().await;
    db::is_db_conn_ok(&db).await;

    db.get_schema_registry("sync_server::db::schema::*")
        .sync(&db)
        .await
        .expect("Failed to sync schema with database");
    info!("DB schema synced");

    let app_state: AppState = AppState {
        db,
        admin_token: util::get_random_string_s(),
    };
    info!("Instnace admin token: {}", app_state.admin_token);

    let app: Router = Router::new()
        .route("/version", get(routes::version::version))
        .nest("/api", api::build_api_router(app_state.clone()))
        // TODO: move to config
        .layer(TraceLayer::new_for_http())
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
