use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseTransaction, DbErr, EntityTrait, TransactionTrait,
};
use tracing::{error, warn};

use crate::{
    AppState,
    db::schema::{machine_path, tracked_file},
    routes::types::{
        create_synced_file::{CreateManySyncedFileReq, CreateManySyncedFileRes},
        generic_internal_err::{InternalErrorCode, InternalErrorRes},
    },
    util::get_user_by_username,
};

pub async fn create_many(
    State(state): State<AppState>,
    Json(input): Json<CreateManySyncedFileReq>,
) -> Result<(StatusCode, Json<CreateManySyncedFileRes>), (StatusCode, Json<InternalErrorRes>)> {
    // Ensure that the user exists
    let user = get_user_by_username(&input.username, &state).await?;

    state
        .db
        .transaction::<_, (), DbErr>(|txn: &DatabaseTransaction| {
            Box::pin(async move {
                for file in input.files {
                    tracked_file::ActiveModel {
                        custom_name: Set(file.custom_name),
                        file_last_modified: Set(file.file_state.file_last_modified),
                        hash: Set(i64::from_ne_bytes(file.file_state.hash.to_ne_bytes())),
                        id: Set(file.id),
                        user_id: Set(user.id),
                    }
                    .insert(txn)
                    .await?;

                    let paths: Vec<machine_path::ActiveModel> = file
                        .file_path_per_machine
                        .iter()
                        .map(|(machine_id, path)| machine_path::ActiveModel {
                            file_id: Set(file.id),
                            machine_id: Set(machine_id.to_owned()),
                            path: Set(path.to_string_lossy().to_string()),
                        })
                        .collect();
                    machine_path::Entity::insert_many(paths).exec(txn).await?;
                }

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

    //TODO: Upload the file to s3 and then store the s3 url in the db
    warn!("TODO: Upload the file to s3");
    Ok((StatusCode::CREATED, Json(CreateManySyncedFileRes {})))
}
