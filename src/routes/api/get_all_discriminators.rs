use axum::{Json, extract::State, http::StatusCode};
use sea_orm::ModelTrait;
use tracing::error;

use crate::{
    AppState,
    db::schema::{tracked_file, user},
    routes::api::types::{
        generic_internal_err::{InternalErrorCodes, InternalErrorRes},
        get_all_discriminators::{GetAllDiscriminatorsReq, GetAllDiscriminatorsRes},
    },
};

/**
 * Get all discriminators for a given user
 */
pub async fn get_all_discriminators(
    State(state): State<AppState>,
    Json(input): Json<GetAllDiscriminatorsReq>,
) -> Result<(StatusCode, Json<GetAllDiscriminatorsRes>), (StatusCode, Json<InternalErrorRes>)> {
    let user = user::Entity::find_by_username(&input.username)
        .one(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding tracked files: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(InternalErrorCodes::InternalDBError)),
            )
        })?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(
                    InternalErrorCodes::NoSuchUserFoundError,
                )),
            ));
        }
    };

    let tracked_files = user
        .find_related(tracked_file::Entity)
        .all(&state.db)
        .await
        .map_err(|e| {
            error!("Error finding tracked files: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(InternalErrorCodes::InternalDBError)),
            )
        })?;

    Ok((
        StatusCode::OK,
        Json(GetAllDiscriminatorsRes {
            discriminators: tracked_files
                .iter()
                .map(|e| e.discriminator.to_owned())
                .collect(),
        }),
    ))
}
