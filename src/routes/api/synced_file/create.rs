use axum::{Json, extract::State, http::StatusCode};
use sea_orm::EntityTrait;
use tracing::error;

use crate::{
    AppState,
    db::schema::{tracked_file, user},
    routes::types::{
        create_synced_file::{CreateDiscrimReq, CreateDiscrimRes},
        generic_internal_err::{InternalErrorCode, InternalErrorRes},
    },
};

pub async fn create(
    State(state): State<AppState>,
    Json(input): Json<CreateDiscrimReq>,
) -> Result<(StatusCode, Json<CreateDiscrimRes>), (StatusCode, Json<InternalErrorRes>)> {
    let user = user::Entity::find_by_username(&input.username)
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding a user: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(InternalErrorCode::InternalDBError)),
            )
        })?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(
                    InternalErrorCode::NoSuchUserFoundError,
                )),
            ));
        }
    };

    // If the file already exists, return an error we shouldn't be creating files with the same id
    match tracked_file::Entity::find_by_id(input.file.id)
        .one(&state.db)
        .await
    {
        Ok(file) => match file {
            Some(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(InternalErrorRes::new(InternalErrorCode::FileAlreadyExists)),
                ));
            }
            None => (),
        },
        Err(e) => {
            error!("Error finding file: {:?}", e);
            return Err((
                StatusCode::BAD_REQUEST,
                Json(InternalErrorRes::new(InternalErrorCode::InternalDBError)),
            ));
        }
    }

    Ok((StatusCode::OK, Json(CreateDiscrimRes {})))
}
