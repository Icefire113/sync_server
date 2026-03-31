use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    let db = db::establish_connection().await;
    db::is_db_conn_ok(&db).await;

    db.get_schema_builder()
        .register(db::schema::user::Entity)
        .register(db::schema::tracked_file::Entity)
        .sync(&db)
        .await
        .unwrap();

    let app_state = AppState {
        db,
        admin_token: util::get_random_string_s(),
    };
    info!("!!! Instnace admin token: {}", app_state.admin_token);

    let app: Router = Router::new()
        .route("/version", get(routes::version::version))
        .route(
            "/api/query_discrim/{discrim}",
            get(routes::api::query_discrim::query_discrim),
        )
        .route(
            "/api/create_user",
            post(routes::api::create_user::create_user),
        )
        .route(
            "/api/get_all_discriminators",
            get(routes::api::get_all_discriminators::get_all_discriminators),
        )
        .with_state(app_state);

    axum::serve(TcpListener::bind("0.0.0.0:3000").await.unwrap(), app)
        .await
        .unwrap();
}
