use std::{env, path::Path, sync::Arc};

use anyhow::{Context, anyhow};
use argon2::{
    Argon2, PasswordHash, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use aws_config::Region;
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
    storage::{StorageProvider, local::LocalStorage, s3::S3Storage},
};

mod config;
mod db;
mod middleware;
mod routes;
mod storage;
mod util;

#[derive(Debug, Clone)]
struct AppState {
    pub db: DatabaseConnection,
    pub admin_token_hash: String,
    pub storage: Arc<dyn StorageProvider>,
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

    match &config.storage_backend {
        config::StorageBackend::Local(local_storage_config) => {
            // ensure local storage dir exists, is writable, and is a directory
            if !Path::new(&local_storage_config.path).try_exists()? {
                return Err(anyhow!("Local storage dir does not exist"));
            }
            if !Path::new(&local_storage_config.path).is_dir() {
                return Err(anyhow!("Local storage dir is not a directory"));
            }
        }
        config::StorageBackend::S3(s3_storage_config) => {
            let sdk_config: aws_config::SdkConfig = aws_config::from_env()
                .endpoint_url(
                    s3_storage_config
                        .endpoint_url
                        .clone()
                        .context("Storage config must specify `endpoint_url` when using s3")?,
                )
                .region(Region::new(s3_storage_config.region.clone().context(
                    "Storage config must specify `region` when using s3",
                )?))
                .load()
                .await;

            let s3_config = aws_sdk_s3::config::Builder::from(&sdk_config)
                .force_path_style(true)
                .build();

            let s3_client = aws_sdk_s3::Client::from_conf(s3_config);
            let bucket_name = s3_storage_config.bucket.clone();
            s3_client
                .head_bucket()
                .bucket(&bucket_name)
                .send()
                .await
                .context(anyhow!(
                    "Failed to head bucket {bucket_name}, does it exist and do we have access to it?"
                ))?;

            let probe_key = format!(".probe/{}", uuid::Uuid::new_v4());
            let probe_bytes = b"sync_server permission probe";
            s3_client
                .put_object()
                .bucket(&bucket_name)
                .key(&probe_key)
                .body(aws_sdk_s3::primitives::ByteStream::from_static(probe_bytes))
                .send()
                .await
                .context(anyhow!(
                    "Failed to write probe object {probe_key}, do we have write access to bucket {bucket_name}?"
                ))?;

            let fetched = s3_client
                .get_object()
                .bucket(&bucket_name)
                .key(&probe_key)
                .send()
                .await
                .context(anyhow!(
                    "Failed to read probe object {probe_key}, do we have read access to bucket {bucket_name}?"
                ))?;
            let fetched_bytes = fetched
                .body
                .collect()
                .await
                .context(anyhow!("Failed to read probe object body"))?
                .into_bytes();
            if fetched_bytes.as_ref() != probe_bytes {
                return Err(anyhow!(
                    "Probe object content did not match what was written"
                ));
            }

            s3_client
                .delete_object()
                .bucket(&bucket_name)
                .key(&probe_key)
                .send()
                .await
                .context(anyhow!("Failed to delete probe object {probe_key}"))?;
            info!("S3 storage backend ok, bucket {bucket_name} is accessible and writable");
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
        storage: match &config.storage_backend {
            config::StorageBackend::Local(local_storage_config) => {
                Arc::new(LocalStorage::new(local_storage_config)) as Arc<dyn StorageProvider>
            }
            config::StorageBackend::S3(s3_storage_config) => {
                Arc::new(S3Storage::new(s3_storage_config).await) as Arc<dyn StorageProvider>
            }
        },
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
