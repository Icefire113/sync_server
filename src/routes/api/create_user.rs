use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use tracing::error;

use crate::{AppState, db::schema::user, util};

#[derive(serde::Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, StatusCode> {
    util::assert_auth(&state, &headers)?;

    let user = user::ActiveModel {
        username: Set(input.username),
        ..Default::default()
    };
    user.insert(&state.db).await.map_err(|e| {
        error!("Error creating user: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, "created"))
}
