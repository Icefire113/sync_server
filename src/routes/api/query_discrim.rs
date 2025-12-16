use axum::{extract::Path, response::IntoResponse};
use tracing::info;

pub async fn query_discrim(Path(discrim): Path<String>) -> impl IntoResponse {
    info!("discrim: {}", discrim);
    "discrim"
}
