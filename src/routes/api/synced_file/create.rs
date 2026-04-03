use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, TransactionTrait,
};
use tracing::{error, warn};

use crate::{
    AppState,
    db::schema::{machine_path, tracked_file, user},
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

    // Ensure that the user exists
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

    // If the file already exists, return an error as we shouldn't be creating files with the same id
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
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(InternalErrorCode::InternalDBError)),
            ));
        }
    }

    //TODO: Upload the file to s3
    warn!("TODO: Upload the file to s3");

    state
        .db
        .transaction::<_, (), DbErr>(|txn: &DatabaseTransaction| {
            Box::pin(async move {
                tracked_file::ActiveModel {
                    custom_name: Set(input.file.custom_name),
                    file_last_modified: Set(input.file.file_state.file_last_modified),
                    hash: Set(i64::from_ne_bytes(input.file.file_state.hash.to_ne_bytes())),
                    id: Set(input.file.id),
                    user_id: Set(user.id),
                }
                .insert(txn)
                .await?;

                let paths: Vec<machine_path::ActiveModel> = input
                    .file
                    .file_path_per_machine
                    .iter()
                    .map(|(machine_id, path)| machine_path::ActiveModel {
                        file_id: Set(input.file.id),
                        machine_id: Set(machine_id.to_owned()),
                        path: Set(path.to_string_lossy().to_string()),
                    })
                    .collect();
                machine_path::Entity::insert_many(paths).exec(txn).await?;

                Ok(())
            })
        })
        .await
        .map_err(|e| {
            error!("Failed to insert file and machine_ids: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(InternalErrorRes::new(InternalErrorCode::InternalDBError)),
            )
        })?;

    Ok((StatusCode::CREATED, Json(CreateDiscrimRes {})))
}
