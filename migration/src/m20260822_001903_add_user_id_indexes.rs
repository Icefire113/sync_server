use entity::{access_token, tracked_file};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260822_001903_add_user_id_indexes"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Index FK columns so per-user lookups don't scan; drop the global
        // files.name index since every file query filters by user first.
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-files-user_id")
                    .table(tracked_file::Entity)
                    .col(tracked_file::Column::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-access_tokens-user_id")
                    .table(access_token::Entity)
                    .col(access_token::Column::UserId)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-files-name")
                    .table(tracked_file::Entity)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .if_not_exists()
                    .name("idx-files-name")
                    .table(tracked_file::Entity)
                    .col(tracked_file::Column::Name)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-access_tokens-user_id")
                    .table(access_token::Entity)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_index(
                Index::drop()
                    .if_exists()
                    .name("idx-files-user_id")
                    .table(tracked_file::Entity)
                    .to_owned(),
            )
            .await
    }
}
