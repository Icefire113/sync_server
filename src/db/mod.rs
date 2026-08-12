use std::{env, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::debug;

pub async fn establish_connection() -> DatabaseConnection {
    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL should be set in env");

    // DB connection options
    let mut db_conn_opts = ConnectOptions::new(db_uri);
    db_conn_opts.connect_timeout(Duration::from_secs(5));

    Database::connect(db_conn_opts)
        .await
        .expect("FAILED TO CONNECT TO DATABASE")
}

pub async fn is_db_conn_ok(db: &DatabaseConnection) {
    assert!(db.ping().await.is_ok());
    debug!("Connected to database")
}
