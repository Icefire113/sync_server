use std::{env, path::Path};

use anyhow::{Context, anyhow};
use argon2::{
    Argon2, PasswordHash, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Router, routing::get};
use dotenvy::dotenv;
use migration::MigratorTrait;
use sea_orm::DatabaseConnection;
use tokio::net::TcpListener;

#[cfg(feature = "response-compression")]
use tower_http::compression::CompressionLayer;
use tower_http::trace::TraceLayer;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    config::Config,
    routes::api::{self},
};

mod config;
mod db;
mod middleware;
mod routes;
mod util;

#[derive(Debug, Clone)]
struct AppState {
    pub db: DatabaseConnection,
    pub admin_token_hash: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(true)
                .with_line_number(true)
                .with_target(true),
        )
        .try_init()
        .unwrap();
    dotenv().ok();
    info!("Starting sync_server ver: {}", env!("CARGO_PKG_VERSION"));
    let config: Config =
        Config::create_if_not_exists().context(anyhow!("Failed to create/ parse config"))?;

    match config.storage_backend {
        config::StorageBackend::Local(local_storage_config) => {
            // ensure local storage dir exists, is writable, and is a directory
            if !Path::new(&local_storage_config.path).try_exists()? {
                return Err(anyhow!("Local storage dir does not exist"));
            }
            if !Path::new(&local_storage_config.path).is_dir() {
                return Err(anyhow!("Local storage dir is not a directory"));
            }
        }
        config::StorageBackend::S3(_s3_storage_config) => {
            // TODO: Check if we can access the bucket, and read/ write to it
        }
    };

    let db: DatabaseConnection = db::establish_connection().await;
    db::is_db_conn_ok(&db).await;

    migration::Migrator::up(&db, None)
        .await
        .context(anyhow!("Failed to run db migrations"))?;
    info!("DB migrations applied");

    let admin_token_hash: String = match config.admin_token {
        Some(token) => PasswordHash::new(&token)
            .context(anyhow!("Failed to parse saved admin token hash"))?
            .to_string(),
        None => {
            let tok: String = util::get_random_string_s();
            info!("Admin token not provided, generating random token");
            info!("Instnace admin token: {}", tok);
            Argon2::default()
                .hash_password(tok.as_bytes(), &SaltString::generate(&mut OsRng))
                .context(anyhow!("Failed to hash generated admin token"))?
                .to_string()
        }
    };

    let app_state: AppState = AppState {
        db,
        admin_token_hash,
        // TODO: Construct storage provider object and shove it in app state
    };

    let app: Router<AppState> = Router::new()
        .route("/version", get(routes::version::version))
        .nest("/api", api::build_api_router(app_state.clone()));
    #[cfg(feature = "response-compression")]
    let app: Router<AppState> = app.layer(CompressionLayer::new().br(true).gzip(true).zstd(true));

    let app: Router<AppState> = if config.log_http_requests {
        app.layer(TraceLayer::new_for_http())
    } else {
        app
    };

    let app: Router = app.with_state(app_state);

    let addr = format!(
        "{}:{}",
        env::var("HOST").unwrap_or_else(|e| {
            error!(
                "Error getting HOST from env: {:?}, defaulting to 0.0.0.0",
                e
            );
            "0.0.0.0".into()
        }),
        env::var("PORT").unwrap_or_else(|e| {
            error!("Error getting PORT from env: {:?}, defaulting to 3000", e);
            "3000".into()
        })
    );

    info!("Server listening on {}", addr);

    axum::serve(TcpListener::bind(addr).await.unwrap(), app).await?;
    Ok(())
}
